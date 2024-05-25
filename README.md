# Tracker

A simple CLI to toggle current time entry in https://track.toggl.com/timer

## Authentication

Authentication is via API Token. The CLI expects for it to be in `TOGGLE_TRACK_TOKEN` environment
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
running

$ tracker
stopped

$ tracker status
stopped
```
