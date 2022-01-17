---
title: "På tide å droppe Redux - og all annen global state"
talk_type: "Lightning talk"
authors:
    - Torry Tufteland
---
Jeg ønsker å vise et lasteikon mens jeg venter på data, et øyeblikk bare, jeg må først sette opp:

- reducers
- konstanter
- actions
- operations
- dispatchers
- middlewares for å støtte async (som om vi lever i 2011). Var det thunks eller sagas igjen?
- connectors
- ...og mer. Manuelt. Hver gang jeg skal hente noe data!?

Grøss og gru! Kanskje jeg skulle blitt backendutvikler likevel? Kanskje jQuery var veien å gå? Hvis dette er moderne teknologi, hva er da meningen med livet? 

Fortvil ikke, o du villfarne sjel, det finnes en bedre måte.

La meg vise deg react-query! I denne lyntalen skal jeg vise deg hvorfor react-query er den beste måten å hente data på, og hvordan det hjelper det med å holde appen din synkronisert med server-staten din.

Nok et state management bibliotek tenker du kanskje? Tja, egentlig ikke. Hvorfor det a'? Kom og se da vel!
