import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property string title: ""
    property string subtitle: ""
    property int value: 0
    property int minimum: 0
    property int maximum: 100
    property string suffix: ""
    signal decrement()
    signal increment()

    height: 86

    RowLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 14

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 8

            RowLayout {
                Layout.fillWidth: true

                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2

                    Text {
                        text: root.title
                        color: Theme.textPrimary
                        font.pixelSize: 16
                        font.weight: Font.Medium
                    }

                    Text {
                        text: root.subtitle
                        color: Theme.textSecondary
                        font.pixelSize: 12
                    }
                }

                Text {
                    text: root.value + root.suffix
                    color: Theme.accent2
                    font.pixelSize: 16
                    font.weight: Font.DemiBold
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 6
                radius: 3
                color: Qt.rgba(0.24, 0.32, 0.52, 0.56)

                Rectangle {
                    width: parent.width * ((root.value - root.minimum) / Math.max(1, root.maximum - root.minimum))
                    height: parent.height
                    radius: parent.radius
                    color: Theme.accent

                    Behavior on width {
                        NumberAnimation { duration: Theme.animationMedium; easing.type: Easing.OutCubic }
                    }
                }
            }
        }

        IconButton {
            Layout.preferredWidth: 36
            Layout.preferredHeight: 36
            label: "-"
            onClicked: root.decrement()
        }

        IconButton {
            Layout.preferredWidth: 36
            Layout.preferredHeight: 36
            label: "+"
            onClicked: root.increment()
        }
    }
}
