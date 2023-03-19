---
title: "Hvordan gjøres en autentisering med OpenID Connect sånn egentlig"
talk_type: "Workshop (1.5 hours)"
type: talk
authors:
    - Øyvind Kallevik Grutle
    - Isak Eriksen Bjørn
---
Du har kanskje oppdaget at flere nettsider og apper de siste årene lar deg logge inn med tredjeparts aktører, som Google, Facebook, Vipps, osv. I denne workshopen skal vi gå gjennom hvordan dette gjøres i praksis med å ta en titt under panseret til OpenID Connect (OIDC) standarden, og hvordan den åpner opp for å gi en tredjeparts aktør ansvaret for å gjennomføre autentiseringen av brukeren.

Autentisering med OIDC er noe som vanligvis bør løses med å bruke biblioteker. I denne workshopen kommer vi derimot til å gjøre alt manuelt for å få en grundig forståelse for hvordan flyten fungerer.

Målgruppen for denne workshopen er utviklere som ikke har noen erfaring med OIDC fra før, eller utviklere som har litt erfaring men ønsker en dypere forståelse av flytene. 
Vi har laget støttekode i Python, men det er ingen problemer å enten løse oppgaven i et annet språk eller med cURL, Postman, osv.
