use std::borrow::Cow;
use std::marker::PhantomData;

use crate::{
    parser::types::Field, registry, registry::Registry, ContextSelectionSet, InputType,
    InputValueError, InputValueResult, OutputType, Positioned, Scalar, ScalarType, ServerResult,
    Value,
};

pub struct RawType<T>(String, PhantomData<T>);


impl<T> RawType<T> {
    pub fn new(data: String) -> Self {
        Self(data, PhantomData)
    }
}

/// The `String` scalar type represents textual data, represented as UTF-8
/// character sequences. The String type is most often used by GraphQL to
/// represent free-form human-readable text.
impl<T: InputType> InputType for RawType<T> {
    type RawValueType = Self;

    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!("{}", T::qualified_type_name()))
    }

    fn qualified_type_name() -> String {
        format!("{}", T::qualified_type_name())
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        T::create_type_info(registry);
        Self::qualified_type_name()
    }

    fn parse(value: Option<Value>) -> InputValueResult<Self> {
        match value.unwrap_or_default() {
            Value::Raw(v) => Ok(RawType::new(v)),
            value => Ok(InputType::parse(Some(value)).map_err(InputValueError::propagate)?),
        }
    }

    fn to_value(&self) -> Value {
        Value::Raw(self.0.clone())
    }

    fn as_raw_value(&self) -> Option<&Self::RawValueType> {
        Some(self)
    }
}

impl<T: OutputType> OutputType for RawType<T> {
    fn type_name() -> Cow<'static, str> {
        Cow::Owned(format!("{}", T::qualified_type_name()))
    }

    fn qualified_type_name() -> String {
        format!("{}", T::qualified_type_name())
    }

    fn create_type_info(registry: &mut registry::Registry) -> String {
        T::create_type_info(registry);
        <RawType<T> as OutputType>::qualified_type_name()
    }

    async fn resolve(
        &self,
        _: &ContextSelectionSet<'_>,
        _field: &Positioned<Field>,
    ) -> ServerResult<Value> {
        Ok(Value::Raw(self.0.clone()))
    }
}
