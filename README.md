# give

A logic engine designed to give you things based on what you ask for

## Implementation

`give` only has one core primitive: types. Types in `give` can either be present or not present.

In `give`, all operations happen through two types of conversions: implicit and explicit. An implicit conversion can happen without it being explicitly asked for by the user. Explicit conversions can only happen when a user specifies them.