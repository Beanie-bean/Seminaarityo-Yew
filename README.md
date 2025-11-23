# Seminaarityö sovellus - Rust Yew

###  Tavoitteet ja toteutus
Seminaarityöni tavoitteena on luoda yksinkertainen web-sovellus käyttäen Rust ohjelmointikieltä ja Yew sovelluskehystä. Sovellus on "todo" eli tehtävienhallintasovellus, jossa käyttäjä pystyy lisäämään, muokkaamaan, poistamaan ja tarkastelemaan omia tehtäviään. Tähän raporttiin tulen kirjaamaan kokemuksiani sovelluksen kehittämisen eri vaiheista. 

Olin ennen projektia opiskellut Rust ohjelmointikieltä, mutta Yew oli minulle täysin uusi. Yew käyttää WebAssembly teknologiaa, joka antaa mahdollisuuden kirjoittaa koodia muillakin kielillä kuin JavaScriptilla ja ajaa koodin verkkoselaimessa. Tässä tapauksessa Rust kielellä.

Apuna sovelluksen kehittämiseen käytin Yew:n ja Rustin dokumentaatiota sekä virallista Yew:n esimerkki sovellusta [todomvc](https://github.com/yewstack/yew/tree/master/examples/todomvc) yew/examples repositoriosta GitHubissa.

### Sovelluksen kehittäminen

Pelkästään dokumentaation avulla oli vaikea päästä alkuun ja moni tutoriaaleista joita löysin sisälsivät vanhaa tietoa. Tapoja toteuttaa sovellus löytyi myös monia. Yew:n virallista todo sovellusesimerkkiä on päivitetty Yew:n eri päivitysten tultua, joten päätin käyttää sitä pohjana omalle sovellukselleni. Dokumentaatiossakin eri aiheiden kohdissa suositeltiin katsomaan joitain virallisia Yew:n esimerkki sovelluksia.

Aloitin käymällä läpi Yew:n dokumentaation [ohjeen](https://yew.rs/docs/tutorial) Yew projektin aloittamiselle. Varsinaisen sovelluksen koodin kirjoittamisen aloitin esimerkkisovellusta pohjana käyttäen ja etsimällä dokumentaatiosta tietoa esimerkin käyttämistä tavoista.

Yew:n käyttäminen oli varsinkin alkuun kovin hankalaa, sillä kaikki Rust kielen ominaisuudet eivät olleet minulle vielä tuttuja. Joissain asioissa Reactin aikaisempi tuntemus auttoi ja huomasin samanlaisuuksia. Esim Yew:ssä html! macro vastaa JavaScriptin JSX-syntaksin käyttöä Reactissa.

CRUD-operaatioiden toteuttaminen lähti lopulta sujumaan tutkittuani ja ymmärrettyäni esimerkkisovelluksen koodia sekä Rust ja Yew dokumentaatioita. Kaikkia esimerkkisovelluksen ominaisuuksia en toteuttanut, sillä halusin pitää sovelluksen yksinkertaisena. Vasta sitten kun olin saanut lisättyä kaikki toiminnot lisäsin tyylin sovellukselle itse tekemälläni css tiedostolla. 

### Lopputulos

Sovelluksesta tuli lopulta toimiva ja tavoitteita vastaava. Opin uusia asioita Rust kielestä sekä tutustuin Yew sovelluskehyksen käyttöön.

Jatkokehitys ideana voisi olla todo:n lisäämis ja muokkaamis tekstikenttien viereen "Add" nappien luonti. Tällä hetkellä lisääminen tapahtuu painamalla "Enter" nappia, mutta visuaalisen napin lisääminen tekisi sovelluksen käytöstä selkeämmän saavutettavamman. 

### Lähteet
Klabnik, S., Nichols, C., & Krycho, C. The Rust Programming Language. https://doc.rust-lang.org/book/. Luettu: 20.11.2025

Vp, S. 2025. Yew: The Rust Web Framework That’s Changing the Game. Luettavissa: https://medium.com/solo-devs/yew-the-rust-web-framework-thats-changing-the-game-e4ce9a31b923. Medium.

Yew. https://yew.rs. Luettu: 19.11.2025.

Yew. https://yew.rs/docs/getting-started/introduction. Luettu: 18.11.2025.

Yew Stack. 2025. todomvc. https://github.com/yewstack/yew/tree/master/examples/todomvc. GitHub.
