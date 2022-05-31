# Placeholder na chadowy tytuł

## Autorzy
[Katarzyna Kloc](https://github.com/KatKlo) - kk429317@students.mimuw.edu.pl\
[Patryk Bundyra](https://github.com/PBundyra) - pb429159@students.mimuw.edu.pl

## Opis
Odkąd CLion wypluł pierwsze segfaulty chcieliśmy napisać grę komputerową inspirowaną studencką przygodą na MIMie.\
GigaChadTytuł będzie to gra platformowa, w której chodzi o to, żeby… zdać. Chcemy stworzyć grę inspirowaną Super Mario Bros, ale w mimowskim wydaniu. Student będzie musiał pokonać bugi, wzmonić się pijąc kawę i kto wie, może nawet stawi czoła niektórym prowadzącym...

Z grubsza będziemy wzorować się na [tym tutorialu](https://dev.to/sbelzile/rust-platformer-part-1-bevy-and-ecs-2pci) oraz [oficjalnych przykładach uzycia](https://github.com/bevyengine/bevy/tree/latest/examples) 

## Funkcjonalności
- generowanie losowych map
- ruch gracza po mapie z przeszkodami
- pokonywanie łatwych przeciwników
- zbieranie wzmocnień (np. szybszy ruch/dalszy skok)
- walka z finałowym bossem
- checkpointy

## Propozycja podziału na części
W pierwszej części stworzymy grę opartą na tutorialu z podstawowymi funkcjonalnościami takimi jak:
- generowanie mapy
- generowanie przeciwnikow
- interakcja z przeciwnikami
- wzmocnienia
- menu początowe

W drugiej części dodamy:
- nowy rodzaj łatwych przeciwników
- finałowego bossa
- podział na poziomu na checkpointy z rosnącym poziomem trudności
- menu końcowe
- efekty dzwiękowe
- zdeployujemy grę używając WebAssembly
- parę easter eggów ;)

## Biblioteki
- Bevy

## Update po 1 części

Robiąc pierwszą część korzystaliśmy głównie z tych tutoriali:

- https://dev.to/sbelzile/rust-platformer-part-1-bevy-and-ecs-2pci 
- https://www.youtube.com/watch?v=j7qHwb7geIM&ab_channel=JeremyChone 
- https://www.youtube.com/watch?v=Yb3vInxzKGE&list=PL7r-PXl6ZPcCB_9zZFU0krBoGK3y5f5Vt&index=5&ab_channel=JeremyChone

oraz z dokumentacji i przykładów do Bevy oraz Rapiera. 

Choć dużą część kodu wzieliśmy z ww. poradników, to używając najnowszej wersji Bevy musieliśmy przekształcić znaczną ilość kodu. Bardzo miło nam przyznać, że wywiązaliśmy się z wszystkich naszych zapowiedzi i zaimplementowaliśmy wszystkie przewidziane feature’y czyli:
- generowanie mapy
- generowanie przeciwnikow
- interakcja z przeciwnikami
- wzmocnienia
- menu początowe

Widzimy też pole na poprawę i rzeczy które chcemy dodać w kolejnej części projektu. Są to m. in. timery i fizykę wzmocnień (bardzo uciążliwe, bo rapier nie posiada akutalnej dokumentacji), czy dodanie własnych eventów i obsługa pewnych akcji w grze właśnie przez customowe eventy. Duża częśc logiki gry polega na losowaniu. To gdzie zostanie umieszczony przeciwnik, kiedy skoczy, jakie dostaniemy wzmocnienie jest oparte na z góry zadanym prawdopobieństwu. Korzystając z komponentów pogrupowaliśmy odpowiednie struktury, takie jak Enemy, albo Booster, dzięki czemu zapewniliśmy skalowalność rozwiązania i zostawiliśmy sobie furtkę na dodanie kolejnych przeciwników i wzmocnień. Mimo, że żeby zapewnić rozgrywkę na wysokim poziomie potrzeba jeszcze dużo pracy, to gra jest obecnie w niezłym stanie, można w nią sensownie pograć i mieć odrobinę frajdy - gwarantujemy jej zwiększenie w kolejnej części :) 


