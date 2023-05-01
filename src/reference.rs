use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RefOr<T> {
    /// A simple object to allow referencing other components in the specification,
    /// internally and externally.
    ///
    /// The Reference Object is defined by
    /// [JSON Reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03)
    /// and follows the same structure,
    /// behavior and rules. A JSON Reference SHALL only be used to refer to a schema that
    /// is formatted in either JSON or YAML. In the case of a YAML-formatted Schema,
    /// the JSON Reference SHALL be applied to the JSON representation of that schema.
    /// The JSON representation SHALL be made by applying the conversion described
    /// [here](https://www.asyncapi.com/docs/specifications/v2.3.0#format).
    ///
    /// For this specification, reference resolution is done as defined by the
    /// JSON Reference specification and not by the JSON Schema specification.
    Ref(Ref),
    T(T),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Ref {
    /// Reference location of the actual component.
    #[serde(rename = "$ref")]
    pub ref_location: String,
}

impl<T> RefOr<T> {
    pub fn ref_(r: &str) -> Self {
        RefOr::Ref(Ref {
            ref_location: r.to_owned(),
        })
    }
    pub fn boxed_item(item: T) -> RefOr<Box<T>> {
        RefOr::T(Box::new(item))
    }
}

impl<T> RefOr<Box<T>> {
    pub fn unbox(self) -> RefOr<T> {
        match self {
            RefOr::Ref(reference) => RefOr::Ref(reference),
            RefOr::T(boxed) => RefOr::T(*boxed),
        }
    }
}
