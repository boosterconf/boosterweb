---
title: "Explain Postgres EXPLAIN"
talk_type: "Workshop (1.5 hours)"
type: talk
authors:
    - Jørgen Langemyr
---
Har du noen gang lurt på hvorfor sql-spørringene dine går så tregt? Eller skulle ønske du forstod mer om hvordan databasen hentet dataene du spurte etter?

Hva skal vi gjøre
EXPLAIN er et av de viktigste verktøyene man har for å lage gode sql-spørringer mot reslasjonsdatabaser. I denne workshopen skal vi bli bedre kjent med Postgres sin versjon av EXPLAIN, både som læringsverktøy og som hjelpemiddel for optimalisering av sql-spørringer. Vi vil jobbe med realistiske eksempler og lære å tolke dataene EXPLAIN gir oss.
Vi vil også undersøke hvordan EXPLAIN kan avdekke suboptimale spørringer, og hvilke grep man kan gjøre for å forbedre resultatet.
Det vi lærer i denne workshopen vil også være overførbart til andre relasjonsdatabaser, ettersom de vanligvis har en lignende versjon av EXPLAIN.

Hvem er dette for
Utviklere med grunnleggende sql-erfaring, men som ønsker å bli bedre kjent med hvordan databaser utfører spørringer. 

Hva trengs av utstyr/software
Deltakere bes ta med en laptop med docker og git installert, da vi vil benytte oss av et docker image med Postgres og SqlPad.
