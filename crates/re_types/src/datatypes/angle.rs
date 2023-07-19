// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// Angle in either radians or degrees.
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Angle {
    Radians(f32),
    Degrees(f32),
}

impl<'a> From<Angle> for ::std::borrow::Cow<'a, Angle> {
    #[inline]
    fn from(value: Angle) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Angle> for ::std::borrow::Cow<'a, Angle> {
    #[inline]
    fn from(value: &'a Angle) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for Angle {
    type Name = crate::DatatypeName;
    #[inline]
    fn name() -> Self::Name {
        crate::DatatypeName::Borrowed("rerun.datatypes.Angle")
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Union(
            vec![
                Field {
                    name: "_null_markers".to_owned(),
                    data_type: DataType::Null,
                    is_nullable: true,
                    metadata: [].into(),
                },
                Field {
                    name: "Radians".to_owned(),
                    data_type: DataType::Float32,
                    is_nullable: false,
                    metadata: [].into(),
                },
                Field {
                    name: "Degrees".to_owned(),
                    data_type: DataType::Float32,
                    is_nullable: false,
                    metadata: [].into(),
                },
            ],
            None,
            UnionMode::Dense,
        )
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data: Vec<_> = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    datum
                })
                .collect();
            UnionArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(<crate::datatypes::Angle>::to_arrow_datatype()),
                        None,
                    )
                } else {
                    <crate::datatypes::Angle>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                data.iter()
                    .map(|a| match a.as_deref() {
                        None => 0,
                        Some(Angle::Radians(_)) => 1i8,
                        Some(Angle::Degrees(_)) => 2i8,
                    })
                    .collect(),
                vec![
                    NullArray::new(DataType::Null, data.iter().filter(|v| v.is_none()).count())
                        .boxed(),
                    {
                        let (somes, radians): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| matches!(datum.as_deref(), Some(Angle::Radians(_))))
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(Angle::Radians(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let radians_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            {
                                _ = extension_wrapper;
                                DataType::Float32.to_logical_type().clone()
                            },
                            radians.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            radians_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, degrees): (Vec<_>, Vec<_>) = data
                            .iter()
                            .filter(|datum| matches!(datum.as_deref(), Some(Angle::Degrees(_))))
                            .map(|datum| {
                                let datum = match datum.as_deref() {
                                    Some(Angle::Degrees(v)) => Some(v.clone()),
                                    _ => None,
                                };
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let degrees_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            {
                                _ = extension_wrapper;
                                DataType::Float32.to_logical_type().clone()
                            },
                            degrees.into_iter().map(|v| v.unwrap_or_default()).collect(),
                            degrees_bitmap,
                        )
                        .boxed()
                    },
                ],
                Some({
                    let mut radians_offset = 0;
                    let mut degrees_offset = 0;
                    let mut nulls_offset = 0;
                    data.iter()
                        .map(|v| match v.as_deref() {
                            None => {
                                let offset = nulls_offset;
                                nulls_offset += 1;
                                offset
                            }

                            Some(Angle::Radians(_)) => {
                                let offset = radians_offset;
                                radians_offset += 1;
                                offset
                            }

                            Some(Angle::Degrees(_)) => {
                                let offset = degrees_offset;
                                degrees_offset += 1;
                                offset
                            }
                        })
                        .collect()
                }),
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::UnionArray>()
                .ok_or_else(|| crate::DeserializationError::DatatypeMismatch {
                    expected: data.data_type().clone(),
                    got: data.data_type().clone(),
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.datatypes.Angle".into(),
                    source: Box::new(err),
                })?;
            if data.is_empty() {
                Vec::new()
            } else {
                let (data_types, data_arrays, data_offsets) =
                    (data.types(), data.fields(), data.offsets().unwrap());
                let radians = {
                    let data = &*data_arrays[1usize];

                    data.as_any()
                        .downcast_ref::<Float32Array>()
                        .unwrap()
                        .into_iter()
                        .map(|v| v.copied())
                        .collect::<Vec<_>>()
                };
                let degrees = {
                    let data = &*data_arrays[2usize];

                    data.as_any()
                        .downcast_ref::<Float32Array>()
                        .unwrap()
                        .into_iter()
                        .map(|v| v.copied())
                        .collect::<Vec<_>>()
                };
                data_types
                    .iter()
                    .enumerate()
                    .map(|(i, typ)| {
                        let offset = data_offsets[i];

                        if *typ == 0 {
                            Ok(None)
                        } else {
                            Ok(Some(match typ {
                                1i8 => Angle::Radians(
                                    radians
                                        .get(offset as usize)
                                        .ok_or(crate::DeserializationError::OffsetsMismatch {
                                            bounds: (offset as usize, offset as usize),
                                            len: radians.len(),
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        })
                                        .map_err(|err| crate::DeserializationError::Context {
                                            location: "rerun.datatypes.Angle#Radians".into(),
                                            source: Box::new(err),
                                        })?
                                        .clone()
                                        .unwrap(),
                                ),
                                2i8 => Angle::Degrees(
                                    degrees
                                        .get(offset as usize)
                                        .ok_or(crate::DeserializationError::OffsetsMismatch {
                                            bounds: (offset as usize, offset as usize),
                                            len: degrees.len(),
                                            backtrace: ::backtrace::Backtrace::new_unresolved(),
                                        })
                                        .map_err(|err| crate::DeserializationError::Context {
                                            location: "rerun.datatypes.Angle#Degrees".into(),
                                            source: Box::new(err),
                                        })?
                                        .clone()
                                        .unwrap(),
                                ),
                                _ => unreachable!(),
                            }))
                        }
                    })
                    .collect::<crate::DeserializationResult<Vec<_>>>()
                    .map_err(|err| crate::DeserializationError::Context {
                        location: "rerun.datatypes.Angle".into(),
                        source: Box::new(err),
                    })?
            }
        })
    }
}

impl crate::Datatype for Angle {}
