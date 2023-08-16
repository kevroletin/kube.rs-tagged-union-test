use kube_derive::CustomResource;
use kube::core::CustomResourceExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[serde(tag="tag")]
pub enum TaggedUnion {
    Left {
        left_value: String
    },
    Right {
        right_value: String
    }
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
    group = "test.dev",
    version = "v1",
    kind = "TestObject",
    namespaced
)]
#[kube(status = "TestStatus")]
#[serde(rename_all = "camelCase")]
pub struct TestSpec {
    pub val: TaggedUnion
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct TestStatus {
}

fn main() {
    println!("{}", serde_yaml::to_string(&TestObject::crd()).unwrap());
}
