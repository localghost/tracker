# Tracker

A simple CLI to toggle current time entry in https://track.toggl.com/timer

## Authentication

Authentication is via API Token. The CLI expects for it to be in `TOGGL_TRACK_TOKEN` environment
variable.

> [!TIP]
> The API Token can be found at the bottom of the [profile page](https://track.toggl.com/profile) .

## Usage

Running the cli will stop current time entry if there is one running otherwise it will start a
new one.

Running the cli with `status` sub-command will show if there is a time entry running or not.

Example:
```
$ tracker
started

$ tracker status
current: 28 minute(s) and 49 second(s)
today: 5 hour(s), 3 minute(s) and 1 second(s)
week: 37 hour(s) and 23 minute(s)
month: 48 hour(s), 49 minute(s) and 15 second(s)

$ tracker start
already running

$ tracker
stopped

$ tracker status
current: stopped
today: 5 hour(s), 3 minute(s) and 1 second(s)
week: 37 hour(s) and 23 minute(s)
month: 48 hour(s), 49 minute(s) and 15 second(s)

$ tracker stop
already stopped

$ tracker start
started

$ tracker stop
stopped
```
