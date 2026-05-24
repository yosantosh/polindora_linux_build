import QtQuick
import QtQuick.Layouts
import ".."

GlassPanel {
    id: root
    property string label: ""
    property string value: ""
    property string detail: ""

    height: 86
    radius: Theme.radiusMedium

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 16
        spacing: 6

        Text {
            text: root.label
            color: Theme.textSecondary
            font.pixelSize: 12
            font.weight: Font.Medium
        }

        Text {
            text: root.value
            color: Theme.textPrimary
            font.pixelSize: root.value.length > 10 ? 16 : 22
            font.weight: Font.DemiBold
            elide: Text.ElideRight
            maximumLineCount: 1
            Layout.fillWidth: true
        }

        Text {
            text: root.detail
            color: Theme.accent
            font.pixelSize: 11
            elide: Text.ElideRight
        }
    }
}
