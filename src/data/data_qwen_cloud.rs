use crate::{ModelPricing, ProviderPricing};

pub const QWEN_CLOUD: ProviderPricing = ProviderPricing {
	name: "qwen_cloud",
	models: QWEN_CLOUD_MODELS,
};

const QWEN_CLOUD_MODELS: &[ModelPricing] = &[];
