# TODO

## Omskrivningar
* Skriv om hur tiles placeras runt planeten. Det mesta av `Planet` metoder måste skrivas om.
* Implementera bevy_rand för att undvika att återkalla rand::thread_rng()
* ✅ Skriv om hur POI:s fungerar. 
* ✅ Ny mapp i components for POI:s, som stenar etc

## Planetgeneration
* Det ska finnas stenar som exempelvis koppar på planeter som har oändligt med liv. Tar mycket tid att utvinna koppar. Koppar kan användas för kablar. Man ska kunna placera automatiserade borrar på dem. (Dessa stenar ska dyka upp på satelliternas kameror)
* Proceduerlla träd ?
* Skuggor under träd => inga fungerande solpaneler där. MHA POI:s och spegelbild av träd?
* ✅ Implementera vertex baserad planet yta
* ✅ Träd och stenar m.m. ska helst placeras i grupper, eftersom jag tror det kan se bättre ut. Alltså måste man göra något noise system för att göra så att träd spawnas mer i vissa områden och mindre i andra.
* ✅ Implementera metod för Planet att göra raycast ner från en degree. 
  
## Tiles och placering
* ✅ Vissa tiles ska highlighta andra POI:s innan man sätter ut den, exempelvis borren ska highlighta stenar om det är i närheten av borren
* ✅ Bättre placerings animationer (sätts ner i marken)

## Buggar
* ✅ Fixa resolution buggen (exempelvis om man går under 100 kraschar det)

## Elnät
* Kablar ska finnas i olika varianter. Ovanliga stenar ger bättre kablar (inte lika mycket energi försvinner per meter) men tar längre tid att utvinna. 

## Allmänt
* Jag tycker att varje spelare ska ha en raket. Varje raket fungerar som spelarens inventory som man kan ta med till andra planeter.
* Implementera items och drops från träd. 

## Dumma Idéer
* Orbital lasers som den där Minecraft data packet. 

## Multiplayer
* Döpa planeter

## Rymden / solsystemet
* Rymdskrot
* Kometnedfall(ovanliga material men förstör mycket)
