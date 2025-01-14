use super::model::ColorScheme;

#[derive(Debug)]
pub enum PreferencesComponentInput {
	SetColorScheme(ColorScheme),
	ToggleExtended(bool),
}

#[derive(Debug)]
pub enum PreferencesComponentOutput {
	ToggleExtended(bool),
}
