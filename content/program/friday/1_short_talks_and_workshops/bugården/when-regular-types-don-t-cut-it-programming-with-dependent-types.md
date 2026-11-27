---
title: "When regular types don't cut it: programming with dependent types"
talk_type: "Workshop 3h"
type: talk
starts_at: 2027-03-19T09:00:00Z
ends_at: 2027-03-19T12:20:00Z
weight: 1
authors:
    - Elisabeth Stenholm

---
What if your programming language could express exactly what you mean?

There are some data structures that the type systems of ordinary languages simply can't express. For example, the type of a valid questionnaire response depends on the questionnaire itself. In most languages, that relationship can only be enforced at runtime.

Dependent types let us move those guarantees into the type system itself, making illegal states impossible to represent.

In this hands-on workshop you'll learn the core ideas behind dependent types by building real examples in Lean. Along the way you'll see how the same techniques naturally lead to formal verification, proving that your code really does what you intended.
