use crate::{ModelPricing, ProviderPricing};

pub const QWEN_CLOUD: ProviderPricing = ProviderPricing {
	name: "qwen_cloud",
	models: QWEN_CLOUD_MODELS,
};

const QWEN_CLOUD_MODELS: &[ModelPricing] = &[
	ModelPricing {
		name: "qwen3.8-max",
		input_cached: Some(0.25),
		input_normal: 2.0,
		output_normal: 6.0,
		output_reasoning: None,
	},
	ModelPricing {
		name: "qwen3.7-plus",
		input_cached: Some(0.064),
		input_normal: 0.32,
		output_normal: 1.28,
		output_reasoning: None,
	},
];
