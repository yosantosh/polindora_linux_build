import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property string iconText: ""
    property string title: ""
    property string subtitle: ""
    property int value: 0
    property bool highlighted: false
    signal decrement()
    signal increment()

    height: 88
    radius: Theme.radiusMedium
    border.color: highlighted ? Theme.accent2 : Qt.rgba(0.55, 0.68, 0.95, 0.32)

    RowLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 16

        Rectangle {
            Layout.preferredWidth: 48
            Layout.preferredHeight: 48
            radius: 24
            color: Qt.rgba(0.18, 0.18, 0.58, 0.42)
            border.color: highlighted ? Theme.accent2 : Theme.accent
            border.width: 1

            Text {
                anchors.centerIn: parent
                text: root.iconText
                color: Theme.textPrimary
                font.pixelSize: 20
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            spacing: 3

            Text {
                text: root.title
                color: Theme.textPrimary
                font.pixelSize: 17
                font.weight: Font.Medium
            }

            Text {
                text: root.subtitle
                color: Theme.textSecondary
                font.pixelSize: 13
            }
        }

        Text {
            text: root.value
            color: Theme.accent2
            font.pixelSize: 19
            font.weight: Font.DemiBold
        }

        IconButton {
            Layout.preferredWidth: 40
            Layout.preferredHeight: 40
            label: "-"
            onClicked: root.decrement()
        }

        IconButton {
            Layout.preferredWidth: 40
            Layout.preferredHeight: 40
            label: "+"
            onClicked: root.increment()
        }
    }
}
