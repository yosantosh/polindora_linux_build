import QtQuick
import QtQuick.Layouts
import ".."

Item {
    id: root
    property var backend
    property string currentTab: "timer"
    property int analyticsMode: 0
    signal closeRequested()

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 24
        spacing: 18

        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 58

            IconButton {
                Layout.preferredWidth: 44
                Layout.preferredHeight: 44
                label: "?"
            }

            Item { Layout.fillWidth: true }

            Text {
                text: "Settings"
                color: Theme.textPrimary
                font.pixelSize: 24
                font.weight: Font.DemiBold
            }

            Item { Layout.fillWidth: true }

            IconButton {
                Layout.preferredWidth: 44
                Layout.preferredHeight: 44
                label: "x"
                onClicked: root.closeRequested()
            }
        }

        RowLayout {
            Layout.fillWidth: true
            Layout.preferredHeight: 70
            spacing: 16

            Rectangle {
                Layout.preferredWidth: 48
                Layout.preferredHeight: 48
                radius: 24
                color: Qt.rgba(0.16, 0.2, 0.5, 0.44)
                border.color: Theme.accent
                border.width: 1

                Text {
                    anchors.centerIn: parent
                    text: currentTab === "timer" ? "o" : "*"
                    color: Theme.textPrimary
                    font.pixelSize: 20
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                spacing: 4

                Text {
                    text: currentTab === "timer" ? "Session Durations" : pageTitle(currentTab)
                    color: Theme.textPrimary
                    font.pixelSize: 18
                    font.weight: Font.DemiBold
                }

                Text {
                    text: pageSubtitle(currentTab)
                    color: Theme.textSecondary
                    font.pixelSize: 13
                }
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 14
            visible: root.currentTab === "timer"

            StepperRow {
                Layout.fillWidth: true
                iconText: "[]"
                title: "Work Duration"
                subtitle: "Minutes per focus session"
                value: backend.workDurationMinutes
                highlighted: true
                onDecrement: backend.decrementWorkDuration()
                onIncrement: backend.incrementWorkDuration()
            }

            StepperRow {
                Layout.fillWidth: true
                iconText: "u"
                title: "Short Break"
                subtitle: "Minutes for short breaks"
                value: backend.shortBreakMinutes
                onDecrement: backend.decrementShortBreak()
                onIncrement: backend.incrementShortBreak()
            }

            StepperRow {
                Layout.fillWidth: true
                iconText: "^"
                title: "Long Break"
                subtitle: "Minutes for long breaks"
                value: backend.longBreakMinutes
                onDecrement: backend.decrementLongBreak()
                onIncrement: backend.incrementLongBreak()
            }

            StepperRow {
                Layout.fillWidth: true
                iconText: "8"
                title: "Long Break Interval"
                subtitle: "Pomodoros before a long break"
                value: backend.longBreakInterval
                onDecrement: backend.decrementLongBreakInterval()
                onIncrement: backend.incrementLongBreakInterval()
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 14
            visible: root.currentTab === "behavior"

            ToggleRow {
                Layout.fillWidth: true
                iconText: "!"
                title: "Strict Focus"
                subtitle: "Hide pause and skip controls during work sessions"
                checked: backend.strictMode
                onToggled: backend.toggleStrictMode()
            }

            ToggleRow {
                Layout.fillWidth: true
                iconText: ">"
                title: "Auto-start Breaks"
                subtitle: "Start short and long breaks when focus completes"
                checked: backend.autoStartBreaks
                onToggled: backend.toggleAutoStartBreaks()
            }

            ToggleRow {
                Layout.fillWidth: true
                iconText: "^"
                title: "Auto-start Work"
                subtitle: "Start the next focus session after a break ends"
                checked: backend.autoStartWork
                onToggled: backend.toggleAutoStartWork()
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 12
            visible: root.currentTab === "appearance"

            SliderRow {
                Layout.fillWidth: true
                title: "Panel Width"
                subtitle: "Desktop integration bar width"
                value: backend.barWidth
                minimum: 60
                maximum: 300
                suffix: "px"
                onDecrement: backend.decrementBarWidth()
                onIncrement: backend.incrementBarWidth()
            }

            SliderRow {
                Layout.fillWidth: true
                title: "Panel Height"
                subtitle: "Progress capsule thickness"
                value: backend.barHeight
                minimum: 2
                maximum: 12
                suffix: "px"
                onDecrement: backend.decrementBarHeight()
                onIncrement: backend.incrementBarHeight()
            }

            SliderRow {
                Layout.fillWidth: true
                title: "Corner Radius"
                subtitle: "Capsule roundness"
                value: backend.barRadius
                minimum: 0
                maximum: 99
                onDecrement: backend.decrementBarRadius()
                onIncrement: backend.incrementBarRadius()
            }

            SliderRow {
                Layout.fillWidth: true
                title: "Glow Intensity"
                subtitle: "Accent bloom strength"
                value: backend.glowIntensity
                minimum: 0
                maximum: 20
                onDecrement: backend.decrementGlowIntensity()
                onIncrement: backend.incrementGlowIntensity()
            }

            SliderRow {
                Layout.fillWidth: true
                title: "Color Saturation"
                subtitle: "Accent color vividness"
                value: backend.colorSaturationPercent
                minimum: 0
                maximum: 200
                suffix: "%"
                onDecrement: backend.decrementColorSaturation()
                onIncrement: backend.incrementColorSaturation()
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 12
            visible: root.currentTab === "analytics"

            SegmentedControl {
                Layout.fillWidth: true
                currentIndex: root.analyticsMode
                onSelected: function(index) {
                    root.analyticsMode = index
                }
            }

            GridLayout {
                Layout.fillWidth: true
                columns: 2
                columnSpacing: 12
                rowSpacing: 12

                StatCard {
                    Layout.fillWidth: true
                    label: "Sessions"
                    value: backend.analyticsCompletedSessions
                    detail: "completed"
                }

                StatCard {
                    Layout.fillWidth: true
                    label: "Focus"
                    value: backend.analyticsFocusMinutes + "m"
                    detail: backend.analyticsTrendText
                }

                StatCard {
                    Layout.fillWidth: true
                    label: "Completion"
                    value: backend.analyticsCompletionRatePercent + "%"
                    detail: "work sessions"
                }

                StatCard {
                    Layout.fillWidth: true
                    label: "Best Time"
                    value: backend.analyticsBestFocusTime
                    detail: analyticsModeLabel(root.analyticsMode)
                }
            }

            GlassPanel {
                Layout.fillWidth: true
                Layout.preferredHeight: 128

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 8

                    Text {
                        text: "Session History"
                        color: Theme.textPrimary
                        font.pixelSize: 16
                        font.weight: Font.DemiBold
                    }

                    Text {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        text: backend.analyticsHistoryText
                        color: Theme.textSecondary
                        font.pixelSize: 13
                        wrapMode: Text.WordWrap
                        lineHeight: 1.25
                    }
                }
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 12
            visible: root.currentTab === "tasks"

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                StatCard {
                    Layout.fillWidth: true
                    label: "Active"
                    value: backend.taskActiveCount
                    detail: "remaining"
                }

                StatCard {
                    Layout.fillWidth: true
                    label: "Done"
                    value: backend.taskCompletedCount
                    detail: backend.taskTotalCount + " total"
                }
            }

            TaskInputRow {
                Layout.fillWidth: true
                onAccepted: function(title) {
                    backend.createTask(title)
                }
            }

            RowLayout {
                Layout.fillWidth: true
                spacing: 12

                GlassButton {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 52
                    label: "Filter: " + backend.taskFilter
                    onClicked: backend.cycleTaskFilter()
                }

                GlassButton {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 52
                    label: "Complete Top"
                    highlighted: true
                    onClicked: backend.completeTopTask()
                }
            }

            GlassPanel {
                Layout.fillWidth: true
                Layout.fillHeight: true

                Text {
                    anchors.fill: parent
                    anchors.margins: 16
                    text: backend.taskListText
                    color: Theme.textSecondary
                    font.pixelSize: 13
                    wrapMode: Text.WordWrap
                    lineHeight: 1.25
                }
            }
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 12
            visible: root.currentTab === "theme"

            ThemeCard {
                Layout.fillWidth: true
                name: backend.activeThemeName
                description: backend.activeThemeDescription
                accentColor: backend.activeAccentColor
                selected: true
                onClicked: backend.cycleTheme()
            }

            GlassPanel {
                Layout.fillWidth: true
                Layout.preferredHeight: 104

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 16
                    spacing: 12

                    RowLayout {
                        Layout.fillWidth: true

                        Text {
                            Layout.fillWidth: true
                            text: "Accent Color"
                            color: Theme.textPrimary
                            font.pixelSize: 16
                            font.weight: Font.DemiBold
                        }

                        Text {
                            text: backend.activeAccentName
                            color: backend.activeAccentColor
                            font.pixelSize: 13
                            font.weight: Font.Medium
                        }
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        spacing: 10

                        Repeater {
                            model: ["#58a6ff", "#7867ff", "#38d6ff", "#47e39b", "#ffc857", "#ff6bd6"]

                            ColorSwatch {
                                swatchColor: modelData
                                selected: backend.activeAccentColor === modelData
                                onClicked: backend.cycleAccent()
                            }
                        }
                    }
                }
            }

            ToggleRow {
                Layout.fillWidth: true
                iconText: "A"
                title: "Auto Theme Switching"
                subtitle: "Prepared for time-aware light and dark palette changes"
                checked: false
            }
        }

        GlassPanel {
            Layout.fillWidth: true
            Layout.fillHeight: true
            visible: root.currentTab !== "timer"
                && root.currentTab !== "behavior"
                && root.currentTab !== "appearance"
                && root.currentTab !== "analytics"
                && root.currentTab !== "tasks"
                && root.currentTab !== "theme"

            Text {
                anchors.centerIn: parent
                width: parent.width - 56
                text: pageTitle(root.currentTab)
                color: Theme.textPrimary
                horizontalAlignment: Text.AlignHCenter
                font.pixelSize: 20
                font.weight: Font.DemiBold
            }
        }

        BottomTabBar {
            Layout.fillWidth: true
            currentTab: root.currentTab
            onTabSelected: function(tab) {
                root.currentTab = tab
            }
        }
    }

    function pageTitle(tab) {
        if (tab === "behavior") return "Behavior"
        if (tab === "appearance") return "Appearance"
        if (tab === "analytics") return "Analytics"
        if (tab === "tasks") return "Tasks"
        if (tab === "theme") return "Theme"
        return "Timer"
    }

    function pageSubtitle(tab) {
        if (tab === "timer") return "Configure how long each session lasts"
        if (tab === "behavior") return "Define focus rules and automatic transitions"
        if (tab === "appearance") return "Tune glass, glow, and panel density"
        if (tab === "analytics") return "Review focus history and trends"
        if (tab === "tasks") return "Manage task workflow and priorities"
        if (tab === "theme") return "Choose palettes and accent colors"
        return ""
    }

    function analyticsModeLabel(index) {
        if (index === 1) return "weekly view"
        if (index === 2) return "monthly view"
        return "daily view"
    }
}
