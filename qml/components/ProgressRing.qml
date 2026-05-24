import QtQuick
import ".."

Canvas {
    id: root
    property real progress: 0.0
    property color trackColor: Qt.rgba(0.55, 0.68, 0.95, 0.18)
    property color progressColor: Theme.accent
    property real lineWidth: 8

    onProgressChanged: requestPaint()
    onWidthChanged: requestPaint()
    onHeightChanged: requestPaint()

    onPaint: {
        var ctx = getContext("2d")
        ctx.reset()

        var size = Math.min(width, height)
        var center = size / 2
        var radius = center - lineWidth
        var start = -Math.PI / 2
        var end = start + (Math.PI * 2 * Math.max(0, Math.min(progress, 1)))

        ctx.lineWidth = lineWidth
        ctx.lineCap = "round"
        ctx.strokeStyle = trackColor
        ctx.beginPath()
        ctx.arc(center, center, radius, 0, Math.PI * 2)
        ctx.stroke()

        ctx.strokeStyle = progressColor
        ctx.beginPath()
        ctx.arc(center, center, radius, start, end)
        ctx.stroke()
    }
}
