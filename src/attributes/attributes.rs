use binary_layout::LayoutAs;
use enum_iterator::{all, first, last, Sequence};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct TokenAttributes(u128);

impl LayoutAs<u128> for TokenAttributes {
    fn read(v: u128) -> TokenAttributes {
        TokenAttributes(v)
    }
    fn write(v: TokenAttributes) -> u128 {
        v.0
    }
}

pub(crate) trait Attributes {
    type Attrs;
    type Serialized = u128;
    type Deserialized = Vec<<Self as Attributes>::Attrs>;
    type Err = String;

    fn deserialize(
        &self,
        val: <Self as Attributes>::Serialized,
    ) -> Result<<Self as Attributes>::Deserialized, <Self as Attributes>::Err>;
    fn serialize(
        &self,
        attrs: <Self as Attributes>::Deserialized,
    ) -> Result<<Self as Attributes>::Serialized, <Self as Attributes>::Err>;
}

pub(crate) struct SequenceAttributes<L, R> {
    _l: std::marker::PhantomData<L>,
    _r: std::marker::PhantomData<R>,
}

pub(crate) enum SequenceAttributesResult<L, R> {
    L(L),
    R(R),
}

fn cst<T, E>(val: T, err: E) -> Result<u128, E>
where
    T: Copy + TryInto<u128>,
{
    val.try_into().map_err(|_| err)
}

impl<L, R> Attributes for SequenceAttributes<L, R>
where
    L: Copy + Attributes + Sequence + TryInto<u128>,
    R: Copy + Attributes + Sequence + TryInto<u128>,
{
    type Attrs = SequenceAttributesResult<L, R>;
    type Deserialized = Vec<SequenceAttributesResult<L, R>>;

    fn deserialize(
        &self,
        val: <Self as Attributes>::Serialized,
    ) -> Result<<Self as Attributes>::Deserialized, <Self as Attributes>::Err> {
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
            Err(String::from("Attributes cannot overlap"))
        } else {
            let mut li = all::<L>()
                .map(|p| {
                    let u: u128 = p.try_into().map_err(|_| String::from("oh no")).unwrap();
                    if val & u == u {
                        Some(p)
                    } else {
                        None
                    }
                })
                .flatten()
                .map(|v| SequenceAttributesResult::L(v));
            let mut ri = all::<R>()
                .map(|p| {
                    let u: u128 = p.try_into().map_err(|_| String::from("oh no")).unwrap();
                    if val & u == u {
                        Some(p)
                    } else {
                        None
                    }
                })
                .flatten()
                .map(|v| SequenceAttributesResult::R(v));
            Ok(li
                .chain(ri)
                .collect::<Vec<SequenceAttributesResult<L, R>>>())
        }
    }
    fn serialize(&self, attrs: Vec<SequenceAttributesResult<L, R>>) -> Result<u128, String> {
        let llu = last::<L>()
            .ok_or(String::from("invalid attributes"))?
            .try_into()
            .map_err(|_| String::from("invalid attributes"))?;
        let rlu = first::<R>()
            .ok_or(String::from("invalid attributes"))?
            .try_into()
            .map_err(|_| String::from("invalid attributes"))?;
        if llu >= rlu {
            Err(String::from("Attributes cannot overlap"))
        } else {
            attrs.iter().fold(
                Ok(0u128),
                |acc: Result<u128, String>, attr: &SequenceAttributesResult<L, R>| {
                    if let Ok(a) = acc {
                        let u: u128 = match attr {
                            SequenceAttributesResult::L(v) => {
                                Ok(cst(*v, String::from("invalid attribute"))?)
                                    as Result<u128, String>
                            }
                            SequenceAttributesResult::R(v) => {
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
