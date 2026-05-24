use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new_qml_module(
        QmlModule::new("com.polindora.app").qml_files([
            "qml/Main.qml",
            "qml/Theme.qml",
            "qml/components/GlassPanel.qml",
            "qml/components/GlassButton.qml",
            "qml/components/IconButton.qml",
            "qml/components/ProgressRing.qml",
            "qml/components/TimerOrb.qml",
            "qml/components/StepperRow.qml",
            "qml/components/ToggleRow.qml",
            "qml/components/SliderRow.qml",
            "qml/components/StatCard.qml",
            "qml/components/SegmentedControl.qml",
            "qml/components/TaskInputRow.qml",
            "qml/components/ThemeCard.qml",
            "qml/components/ColorSwatch.qml",
            "qml/components/BottomTabBar.qml",
            "qml/components/SettingsPanel.qml",
        ]),
    )
    .qt_module("Network")
    .files(["src/bridge.rs"])
    .build();
}
