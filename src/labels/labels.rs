use binary_layout::LayoutAs;
use enum_iterator::{all, first, last, Sequence};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TokenLabels(u128);

impl LayoutAs<u128> for TokenLabels {
    fn read(v: u128) -> TokenLabels {
        TokenLabels(v)
    }
    fn write(v: TokenLabels) -> u128 {
        v.0
    }
}

pub(crate) trait Labels {
    type Lbls;
    type Serialized = u128;
    type Deserialized = Vec<<Self as Labels>::Lbls>;
    type Err = String;

    fn deserialize(
        &self,
        val: <Self as Labels>::Serialized,
    ) -> Result<<Self as Labels>::Deserialized, <Self as Labels>::Err>;
    fn serialize(
        &self,
        attrs: <Self as Labels>::Deserialized,
    ) -> Result<<Self as Labels>::Serialized, <Self as Labels>::Err>;
}

pub(crate) struct SequenceLabels<L, R> {
    _l: std::marker::PhantomData<L>,
    _r: std::marker::PhantomData<R>,
}

pub(crate) enum SequenceLabelsResult<L, R> {
    L(L),
    R(R),
}

fn cst<T, E>(val: T, err: E) -> Result<u128, E>
where
    T: Copy + TryInto<u128>,
{
    val.try_into().map_err(|_| err)
}

impl<L, R> Labels for SequenceLabels<L, R>
where
    L: Copy + Labels + Sequence + TryInto<u128>,
    R: Copy + Labels + Sequence + TryInto<u128>,
{
    type Lbls = SequenceLabelsResult<L, R>;
    type Deserialized = Vec<SequenceLabelsResult<L, R>>;

    fn deserialize(
        &self,
        val: <Self as Labels>::Serialized,
    ) -> Result<<Self as Labels>::Deserialized, <Self as Labels>::Err> {
        let llu = {
            if let Some(l) = last::<L>() {
                cst(l, String::from("invalid attributes"))
            } else {
                Err(String::from("invalid attributes"))
            }
        };
        let rlu = {
            if let Some(r) = last::<R>() {
                cst(r, String::from("invalid attributes"))
            } else {
                Err(String::from("invalid attributes"))
            }
        };
        if llu >= rlu {
            Err(String::from("Labels cannot overlap"))
        } else {
            let li = all::<L>()
                .map(|p| {
                    let u: u128 = p.try_into().map_err(|_| String::from("oh no")).unwrap();
                    if val & u == u {
                        Some(p)
                    } else {
                        None
                    }
                })
                .flatten()
                .map(|v| SequenceLabelsResult::L(v));
            let ri = all::<R>()
                .map(|p| {
                    let u: u128 = p.try_into().map_err(|_| String::from("oh no")).unwrap();
                    if val & u == u {
                        Some(p)
                    } else {
                        None
                    }
                })
                .flatten()
                .map(|v| SequenceLabelsResult::R(v));
            Ok(li.chain(ri).collect::<Vec<SequenceLabelsResult<L, R>>>())
        }
    }
    fn serialize(&self, attrs: Vec<SequenceLabelsResult<L, R>>) -> Result<u128, String> {
        let llu = last::<L>()
            .ok_or(String::from("invalid attributes"))?
            .try_into()
            .map_err(|_| String::from("invalid attributes"))?;
        let rlu = first::<R>()
            .ok_or(String::from("invalid attributes"))?
            .try_into()
            .map_err(|_| String::from("invalid attributes"))?;
        if llu >= rlu {
            Err(String::from("Labels cannot overlap"))
        } else {
            attrs.iter().fold(
                Ok(0u128),
                |acc: Result<u128, String>, attr: &SequenceLabelsResult<L, R>| {
                    if let Ok(a) = acc {
                        let u: u128 = match attr {
                            SequenceLabelsResult::L(v) => {
                                Ok(cst(*v, String::from("invalid attribute"))?)
                                    as Result<u128, String>
                            }
                            SequenceLabelsResult::R(v) => {
                                Ok(cst(*v, String::from("invalid attribute"))?)
                                    as Result<u128, String>
                            }
                        }?;
                        Ok(a | u)
                    } else {
                        acc
                    }
                },
            )
        }
    }
}
