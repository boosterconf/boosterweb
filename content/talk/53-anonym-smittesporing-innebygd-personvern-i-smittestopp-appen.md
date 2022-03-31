---
title: "Anonym smittesporing – innebygd personvern i Smittestopp-appen"
talk_type: "Experience report (30 min)"
authors:
    - Henrik Walker Moe
time_slot: "Friday: 10:45 - 12:15"
room: "Kongesalen 2+3"
vimeo_id: 692619792
---
Smittestopp 1 ble avsluttet tidlig på høsten 2020 etter opprop fra fagmiljø og et påfølgende forbud fra Datatilsynet. Da planleggingen av Smittestopp 2 startet, var det basert på Google og Apples rammeverk for kontaktsporing (GAEN). Tilnærmingen med Smittestopp 2 ble radikalt annerledes, med blant annet utvikling som åpen kildekode. Henrik Walker Moe, Tjerand Silde og Martin Strand møttes høsten 2020 gjennom fagdiskusjoner rundt Smittestopp, og ønsket å bidra til å forbedre personvernet i Smittestopp 2 ytterligere.

Resultatet av disse diskusjonene ble programvaremodulen “Anonymous tokens” som er basert på anonymiseringsprotokollen “Privacy Pass”. Den fungerer slik at en bruker får utstedt en signert attest, som kan maskeres fullstendig av brukeren først. Det kan likevel kun brukes én gang, hvor hver bruker mottar en attest som brukes som en identitet ved opplasting av smitte-data. Det er denne anonyme attesten sørger for innebygd personvern ved at den ikke er sporbar tilbake til en enkeltperson.

I dette foredraget tar vi dere gjennom reisen Henrik, Tjerand og Martin hadde i Open Source-samarbeidet med FHI. Vi vil forklare hvordan ideen til “Anonymous tokens” oppstod, hvordan implementasjonen fungerer konseptuelt (inkludert noe kryptografi) og til slutt hvordan anonyme attester ble en del av Smittestopp-appen.

I tillegg vil vi belyse andre praktiske bruksområder for en slik anonymisering ut ifra et personvernsperspektiv og dataminimeringsprinsipp, for eksempel logging og anonyme billetter.
