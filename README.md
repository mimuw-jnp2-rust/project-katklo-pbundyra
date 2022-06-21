# Placeholder na chadowy tytuł

## Autorzy
[Katarzyna Kloc](https://github.com/KatKlo) - kk429317@students.mimuw.edu.pl\
[Patryk Bundyra](https://github.com/PBundyra) - pb429159@students.mimuw.edu.pl

# Part 0
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

# Part 1

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

Widzimy też pole na poprawę i rzeczy które chcemy dodać w kolejnej części projektu. Są to m. in. timery i fizykę wzmocnień (bardzo uciążliwe, bo rapier nie posiada akutalnej dokumentacji), czy dodanie własnych eventów i obsługa pewnych akcji w grze właśnie przez customowe eventy. Duża częśc logiki gry polega na losowaniu. To gdzie zostanie umieszczony przeciwnik, kiedy skoczy, jakie dostaniemy wzmocnienie jest oparte na z góry zadanym prawdopobieństwu. Korzystając z komponentów pogrupowaliśmy odpowiednie struktury, takie jak Enemy, albo Powerup, dzięki czemu zapewniliśmy skalowalność rozwiązania i zostawiliśmy sobie furtkę na dodanie kolejnych przeciwników i wzmocnień. Mimo, że żeby zapewnić rozgrywkę na wysokim poziomie potrzeba jeszcze dużo pracy, to gra jest obecnie w niezłym stanie, można w nią sensownie pograć i mieć odrobinę frajdy - gwarantujemy jej zwiększenie w kolejnej części :) 


# Part 2
## Opis:
Włączcie Sweden i znów poczujcie się jak wtedy,
gdy mając 12 lat w swoim pokoju, przekopywaliście się przez kolejne stone'y,
żeby dostać upragnione diamenty.
Ah, nie, nie musicie tego robić,
bo kolejna wersja „Mario MIM” dzięki swojej wyjątkowej szacie audiowizualnej
przeniesie was do stanu tej błogiej nostalgii.
Wspomnienia przywoływane przez grę będą sięgać również bliższej przeszłości,
gdy jako młodzi kadeci Podstaw Matematyki dostaliście pierwsze oceny,
pierwszy raz zobaczyliście zielony komunikat o poprawnie wykonanych testach w swoich terminalach
czy zamieniliście C++ na Rusta.
Jako studenci MIM-u musicie stawić czoła różnym bugom lub - co gorsza - Valgrindowi…
Aby jakoś przebrnąć do końca kolejnego semestru możecie wzmocnić się kawą lub
nauczyć najbardziej uwielbianego języka na świecie.
Chcecie podzielić się rozgrywką z innymi studentami MIM-u?
Nic prostszego, ponieważ możecie zagrać [online](https://pbundyra.github.io/wasm-deploy/)
i dzięki seedowi wygenerować tę samą mapę, co Twój towarzysz studenckiej podróży.
## Zmiany:
- poprawienie wydajności
- poprawienie fizyki powerup-ów
- dodanie timerów do powerup-ów
- dodanie kolejnych elementów menu
- dodanie seed-u generującego mapę
- poprawienie poruszania się przeciwników
- dodanie kolejnego przeciwnika, który potrafi strzelać
- dodanie efektów dźwiękowych
- poprawienie collider-ów ścian
- deploy gry używając WebAssembly i Github Pages - https://pbundyra.github.io/wasm-deploy/
- dodanie mechaniki poziomów ze zwiększającym się poziomem trudności
- podział poziomów na 3 różne rodzaje
- naprawa bugów
- poprawa stylu i przejrzystości kodu z poprzedniej części
    
## Czego nie udało nam się zrobić i dlaczego:
Nie zrobiliśmy finałowego bossa, ponieważ zmieniliśmy koncepcję stopniowania trudności poziomów oraz co ważniejsze, chcieliśmy dostarczyć kod który będzie zawierał trochę mniej feature’ów, ale za to będzie lepszej jakości. Zrobiliśmy ogromny refactor pierwszej części, korzystając z mechaniki Eventów w Bevy oraz dodaliśmy kolejne ważne Pluginy. Postawiliśmy na jakość, zamiast ilość i po ukończeniu projektu uważamy, że była to lepsza decyzja.
Czy są bugi? Są, ale bardzo rzadko- prawdopodobnie podczas rozgrywki nie będziecie mieli szansy ich nawet zauważyć (więc nie będziemy ich zdradzać), oprócz tych, które sami musicie pokonać. Naprawiliśmy bardzo dużo bugów z poprzedniej części, ale zabrakło zasobów, żeby złapać je wszystkie ;)
## Final thoughts:
Była to nasza pierwsza gra i jesteśmy w niej w pełni dumni. Bardzo dużo się nauczyliśmy, a Bevy okazało się świetną biblioteką, jednak potrzebowaliśmy trochę czasu (1 część projektu) żeby trochę się z nią oswoić. Mamy nadzieję, że będzie wam się w nią przyjemnie grało oraz przede wszystkim, że niejednokrotnie wywoła uśmiech na waszych twarzach.



