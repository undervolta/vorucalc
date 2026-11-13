use crate::slint_generatedAppWindow::SemanticSize;
use serde::{ Deserialize, Deserializer, Serializer };

pub mod size_serde {
	use super::*;

    pub fn serialize<S>(
        value: &SemanticSize,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = match value {
            SemanticSize::Xxs => "xxs",
			SemanticSize::Xs => "xs",
			SemanticSize::Sm => "sm",
			SemanticSize::Md => "md",
			SemanticSize::Lg => "lg",
			SemanticSize::Xl => "xl",
			SemanticSize::Xxl => "xxl"
        };

        serializer.serialize_str(value)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<SemanticSize, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        match value.as_str() {
            "xxs" => Ok(SemanticSize::Xxs),
            "xs" => Ok(SemanticSize::Xs),
            "sm" => Ok(SemanticSize::Sm),
			"md" => Ok(SemanticSize::Md),
			"lg" => Ok(SemanticSize::Lg),
			"xl" => Ok(SemanticSize::Xl),
			"xxl" => Ok(SemanticSize::Xxl),
            _ => Err(serde::de::Error::custom("Unknown size")),
        }
    }
}
