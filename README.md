# rcommunity

A subsystem for building a community on top of existing business.

The letter `r` in `rcommunity` may refer to Rust, the language this library
is written in, or Redis, the persistent layer that data are stored in, or
Reaction, the concept we use to model community behaviors.

## Main Concept

We use the concept **reaction** to represent all behaviors or events in a
community, such as voting, commenting, etc. Events are grouped as different
types with different reaction kind. For each type, all the events are
represented as a collection of 3-tuples: `(user, item, reaction)`.

With `rcommunity`, you can define different reaction types, record
reactions, and retrieve them on your need. You can easily build a community
that fits your own business, whether it contain reactions as simple as
comments, or as complex as voting, tagging, tag voting, and more.

License: MIT
