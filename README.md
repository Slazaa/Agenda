# Agenda
A crossplatform agenda written in Rust

## Examples
Add an event the 23th of April 2023 at noon
Note that the numbers after the '-' are optional
```
agenda add 2023/04/23-12:00:0 "There is an event!"
```

Remove the event the 23th of April 2023 at noon
```
agenda remove 2023/04/23-12
```

Clear all events
```
agenda clear
```

List all events
```
agenda list

2024 = December =
- [25] (0:0.0) - "Christmas!"
```
