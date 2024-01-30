---
title: "Mischief Managed using Design Tokens 🧙 "
talk_type: "Experience report"
type: talk
authors:
    - Abel van Beek

---
Consistency is hard, maintenance is even harder. With 25 years of history, Vizrt products have lived through multiple eras. The challenge has shifted from developing the products to keeping their design and user experience unified and up to date.

Recently, we revamped our design system and have explored ways of propagating these updates throughout our products, across different stacks. We aim to inspire others by sharing the pitfalls and best practices we have discovered during our journey. We will guide the audience through this transition by sharing clear examples of our products from before and after this initiative.

In our process of trial and error, we started out skinning third-party libraries. This resulted in performance and maintenance issues as dependencies changed. Finally, we settled on sharing design tokens among our designs in Figma (using Figma Variables) and linked them directly to their counterparts in internally managed component libraries in Windows Presentation Foundation (WPF) and web applications, leveraging TypeScript and CSS variables to link it all together.

Product development has improved noticeably in speed, ease of design maintenance, and accessibility. The separation between design choices and component implementations enables the use of legacy design token sets that helped us drive adoption of this shiny, new design system. We will show how to use the same concept to maintain the design system and see the changes apply instantly across our applications. 

We believe these experiences can truly benefit others in their design system journeys.
