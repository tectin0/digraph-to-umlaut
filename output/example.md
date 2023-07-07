---
layout: default
title: Tag 2
day: 2
# FIXME: transforms trü to tr"ü" -> because early cutoff in tree search
output: trü
usemathjax: trü
---

<wd>
    <h1>
        AC-Praktikum: Analytik
    </h1>
    <h1>
        Quantitativ (wie viel?) und Qualitativ (welcher Stoff?)
    </h1>
</wd>

<p>Manchmal Frage von Studenten: Warum diese altertümlichen, anmutenden, nass-chemischen (?) Versuche,
    wenn es doch allerhand moderne Sensoren & Analytimethoden gibt? Grund: Wir wollen ja die <u>Chemie
        erlernen & nicht die physikalischen Verfahren</u>. Und dazu ist neben dem
    <u>Handwerkszeug</u>, das wir im Praktikum lernen, auch die dazugehörige <u>Chemie</u>, die das
    <u>beobachtete Verhalten</u> erklärt, wichtig. Die <u>Analytik</u> nimmt dabei eine wichtige Rolle ein,
    denn ohne zu wissen <u>welcher Stoff</u> bei einer Synthese oder auch einer natürlichen Probe vorliegt,
    macht es keinen Sinn beim Verhalten od. seine Eigenschaften untersuchen undverstehen zu wollen.
    <u>Erstmal muss ich wissen was ich für einen Stoff habe</u> und in der Anwendung will ich auch oft
    wissen <u>wieviel</u> ich davon habe, bspw. um eine Reaktion zu verfolgen und zu wissen wann der
    gewünschte Stoff vorliegt.
</p>
<p>
    Das ist übrigens im wesentlich gleich für Chemiestudenten im 1. Jahr Beispiel: <u>Warum sind bestimmte
        Stoffe löslich im wässrigen System od. im Erdreich, warum andere nicht. Warum kommt Al nicht
        elementar vor? aber Gold wohl?</u>
</p>
<h2>
    <w>Quantitativ: a) klassische Methoden und b) physikalische Methoden</w>
</h2>
<p>
    Quantität eines Stoffes in einem Gemenge oder einer Loseung bestimmen. Auch wenn später Automaten für solche
    Fragen
    verwendet werden, kann es gut sein dass diese quasi als <u>Black-Box</u> intern ähnliche Vorgänge ausführen
    bspw.
    Tropfen zählen also Volumen bestimmen wie sie gleich und/oder auch Spektroskopische also physikalische Methoden
    anwenden.
    Am Ende zeigt Ihnen dann der Rechner so und so viel mg an.
</p>
<p>
    Dafür braucht man dann allerdings keine Chemie-Kenntnisse und drückt nur ein paar Knöpfe. Wir konzentrieren
    uns
    also
    im Folgenden auf die klassischen Methoden; wir wollen ja Chemie lernen.<br>
    <u>Woher auch für physikalische Methoden zum kalibrieren</u> natürlich wieder ein chemisches Verständnis
    benötigt
    wird, da ja <u>Nullpunkte</u> und <u>Eichkurven</u> mit dazugehörigen Lösungen erst bestimmt werden müssen,
    damit
    das
    Gerät arbeiten kann.
</p>
<wd>
    <h2>Versuche zu klassischen Methoden</h2>
    <b>Gravimetrie (Gewichtsanalyse)</b><br>
    <b>Titrimetrie (massanalyse bzw. Volumetrie)</b><br>
</wd>

Gravimetrie: Heute aufgrund der elektrischen Waagen viel einfacher als früher \-> nur ein
Versuch - $\ce{Ca2+}$ Gravimetrie nächsten Montag<br>

<wd>
    <h2>masse des Reaktionsprodukts einer Fällungsreaktion</h2>
</wd>
<p>
    Wir schütten zwei Lösungen zusammen, dabei fällt ein Feststoff aus \-> diesen wiegen wir. <br>
    Beispiel $\ce{Ca2+}$ fällen: Wie? Oxalat oder Fluorid. <br>
    Reaktion im Praktikum: <w>$\ce{Cl- + Ag+ -> AgCl v ->[h\nu] Ag + 1/2 Cl2}$</w> <br> \-> Gewichtsaenderung
</p>

<h3>Bedingungen für eine Gravimetrie</h3>
<ul>
    <li>
        <w>Die Fällungs muss quantitativ (vollständig) sein.</w><br>
        d.h. die Löslichkeit muss möglichst gering sein! Wie angehen? <br>

        <w>Restkonzentration in Lösung</w>
        < $10^{-5}$ mol/L $\hat{=} \cdot 40.08 \ \mathrm{g/mol}=0.4 \ \mathrm{mg/L}$ <br>

            d.h. der Fehler der Fällung muss klein gegen die zu bestimmende Substanzmenge sein.
    </li>
    <li>
        <w>Der Niederschlag muss eine konstante und bekannte Zusammensetzung haben.</w> oder durch eine nachfolgende
        Behandlung in sie überführt werden. <br>
        D.h. $\ce{AgCl}$ ist nur bedingt geeignet, da es sich an Licht zersetzt. <br>
        Man müsste im Dunkeln oder aber sehr schnell auswiegen.
    </li>
    <li>
        <w>Der Niederschlag muss in "Wägeform" vorliegen.</w> Ein Product, dass bspw. <u>hygroskopisch</u> ist, also
        wasser aus der Luft zieht, der würde beim Wiegen immer schwerer wiegen. <br>
        Anderes Beispiel für eine Fällung: Trinkwasseraufbereitung <br>
        <wd>
            $\ce{
            $\underset{
            \color{red}{
            \text{aus der Eifel:<br>Trinkwasser}
            }
            }{
            \ce{Fe3+}
            }$
            +
            $\underset{
            \color{red}{
            \text{alkalisch machen z.B. } \ce{NH3}
            }
            }{
            \ce{OH-}
            }$
            ->
            $\underset{
            \substack{
            \text{Fällungsform \-> abfiltrieren} \\
            \color{red}{
            \text{Völlig undefinierte Zusammensetzung} \\
            \text{Schlamm von Hydroxyden}
            }
            }
            }
            {"\ce{Fe2O3 \cdot x H2O}"}$
            ->[\Delta T][\substack{\color{red}{
            \text{Glühen bei 600 $^\circ$C} \\
            \text{im Porzelantiegel} \\
            \text{einige Stunden}
            }
            }] $\underset{
            \text{Wägeform}
            }
            {\ce{Fe2O3}}$
            }$
        </wd>
        Stoichiometrie sagt uns wie die masse auf die gesuchte Substanz umgerechnet werden kann.<br>
        \lightning: Für das Prakitkum eher schwierig, da die 600$^\circ$C sehr genau stimmen müssen,
        sonst bilden sich andere Oxide: $\ce{FeO}$, $\ce{Fe3O4}$. Im Praktikum daher ein einfacheres Experiment \-> u.
        Mo. (?)
    </li>

</ul>

<wd>
    <h2>Titrimetrie</h2>
</wd>
<p>
    Prinzip basierend auf zwei Lösungen:
</p>
<wd>
    <ol>
        <li>
            Lösung <u>bekanntes Volumen</u> und <u>unbekannter Konzentration</u> des zu bestimmenden Stoffes
        </li>
        <li>
            Lösung <u>bekannter Konzentration</u>, bis zur vollständingen Umsetzung (Endpunkt) titrieren.
            Der Vorgang heißt Titration.
        </li>
    </ol>
</wd>
<p>
    Die Reaktion der beiden Stoffe sollte dabei möglichst zügig verlaufen. Beim $\ce{AgCl}$-Beispiel würde man
    solange
    $\ce{Ag+}$ hinzugeben, bis nichts mehr ausfällt.
</p>
<wd>
    Zur besseren Endpunktsbestimmung kann ein Indikator zugegeben werden. Zum Beispiel:<br>
    $\ce{
    $\underset{
    \text{Massloesung}
    }{\ce{OH-}}$
    + H+ ->[\text{pH-Indikator}] H2O
    }$
</wd>
<br>
<br>
<wd>
    <div id="tester" style="width:600px;height:400px;"></div>

    <script>
        var x = linspace(0, 14, 100);
        var y = sigmoid(x);

        TESTER = document.getElementById('tester');

        var pH_plot = {
            x: x,
            y: y,
            mode: 'lines',
            line: {
                color: 'black',
                width: 3,
            },
        };

        var h_line = {
            x: [0, 14],
            y: [7, 7],
            mode: 'lines',
            line: {
                color: 'red',
                width: 2,
                dash: 'dashdot',
            },
        };

        var äp_annotation = {
            xref: "x",
            yref: "y",
            x: 10,
            y: 8,
            text: "Äquivalenzpunkt (AEP)",
            showarrow: false,
        }

        Plotly.newPlot(TESTER, [
            pH_plot,
            h_line,
        ], {
            font: {
                family: 'Arial',
                size: 18,
            },
            showlegend: false,
            xaxis: {
                title: "mL HCl 0.1N",
                range: [0, 15],
                tickvals: [],
            },
            yaxis: {
                title: "pH",
                range: [0, 14],
                tickvals: [7],
                ticktext: [7],
            },
            annotations: [
                äp_annotation,
            ],
            paper_bgcolor: 'rgba(0,0,0,0)',
            plot_bgcolor: 'rgba(0,0,0,0)',

        });
    </script>

</wd>
<p>
    Aus dem Volumen under Konzentration des Titers kann dann die Konzentration in 1) berechnet werden (V ist bekannt).
</p>

<ol type="a">
    <li>
        <w>Säure-Base-Titration</w>: <br>
        Über pH-Wert, relativ leicht durchführbar. Beachten muss man ob man es mit <u>starken oder eher schwachen
            Saueren/Basen</u> durchführt, <u>welchen Indikator</u> man nimmt oder ob z.B. <u>mehrstufige oder
            Puffersysteme</u> vorliegen. <u>Virtuelles Labor!</u> \-> Verweise auf Selbstlerneinheit
    </li>
    <li>
        <w>Fällungstitration</w>: <br>
        Beispiel <u>$\ce{AgCl}$</u>; <u>Endpunkt schwierig</u>, daher indirekt mit einem <u>Indikator der auf $\ce{Cl-}$
            empfindlich ist</u> und anzeigt wenn alles <u>$\ce{Cl-}$ verbraucht </u> ist durch einen <u>Farbumschlag
            $\hat{=}$ Endpunkt</u> bzw. bereits ein klein wenig zu viel (<u>übertitriert</u>); während bei starken
        Säuren/Basen der Umschlag schwierig ist.
    </li>
    <li>
        <w>Redox-Titration</w>: Versuch Mittwoch <br>
        <wd>
            $\ce{
            5 $\overset{\color{red}{+2}}{\ce{Fe}}$ ^{2+} +
            $\overset{\color{red}{+7}}{\ce{Mn}}$ O4- +
            8H+
            ->
            5 $\overset{\color{red}{+3}}{\ce{Fe}}$ ^{3+} +
            $\overset{\color{red}{+2}}{\ce{Mn}}$ ^{2+} +
            4H2O
            }$
        </wd>
        <script type="text/tikz">
            \begin{tikzpicture}
                \tikzstyle{every node}=[font=\LARGE]    
                \node at (0.0, 0.0) {};
                \draw[->, line width=1mm] (0.0, 0.0) -- ++(down:1.0) -- ++(east:8.3) -- (8.3, 0.0);
                \node at (5.4, -0.5) {\textbf{Oxidation: $5\textrm{e}^{\textrm{--}}$}};
                \draw[->, line width=1mm] (3.0, 0.0) -- ++(down:2.0) -- ++(east:8) -- (11.0, 0.0);
                \node at (6.8, -1.5) {\textbf{Reduktion: $5\textrm{e}^{\textrm{--}}$}};
            \end{tikzpicture}
        </script>
        <br>
        Permanganat-Ion ist dunkelviolett, während die anderen ionen in der hohen Verdünnung praktisch farblos sind.
        <br>
        Beim Eintropfen färbt sich die Lösung kurz ins violette entfärbt sich dann wieder komplett. Erst wenn etwas
        zu viel $\ce{MnO4-}$ hineingetropft wurde bleibt die Lösung gefärbt. (Vorsicht: auf Dauer erfolgt Oxidation an
        der Luft.)
    </li>
    <li>
        <w>Komplexometrie</w>: Heute häufig <br>
        <wd>
            $\ce{
            Ca^2+ +
            $\underset{
            \substack{
            \color{red}{
            \text{Markenname:} \\
            \text{Titriplex} \\
            \text{im Protokoll} \\
            \text{wird deprotoniert}
            }
            }
            }
            {\ce{H2Y^2-}}$
            ->[\textrm{z.B. $\ce{NH3}$ Buffer}][\textrm{z.B. $\ce{NaOH}$}]
            CaY^2- +
            2H+
            }$
        </wd>
        <u>Endpunkt über weiteren Farbkomplex</u>, d.h. einen Farbumschlag. Gearbeitet wird im alkalischen, da nur hier
        der Komplex stabil ist und in der Reaktion $\ce{H+}$ entsteht. <br>
    </li>
</ol>
<p>
    <!-- FIXME: check why Massloesung is not converted -->
    <u>Massloesung</u> muss gut eingestellt sein. Entweder selber oder gekauft. Selber ist nicht ganz leicht. Man
    muss eine Lösung einer bekannten Konzentration herstellen. <br>
    Beispiel: Für <u>Salzsäure müsste $\ce{HCl}$-Gas</u> in wasser gelöst werden; das kann man schlecht wiegen
    und es löst sich nur züinem bestimmten Teil nicht vollständig. daher nicht konzentrierte $\ce{HCl}$. Oder man
    nimmt eine Metallsalz-Lösung; diese müsste aber auch erst titriert werden. <br>
    Oder: man verdünnt konzentrierte $\ce{HCl}$; aber ungenau. <br>
    <u>besser:</u> wäre eine Stoff der sich <u>gut löst in wasser</u> und den man <u>gut wiegen</u> kann und
    dessen Lösung <u>lange Zeit stabil</u> ist. <br>
    z.B. Soda also $\ce{Na2CO3}$; das Salz einer schwachen Säure in $\ce{H2O}$ gelöst wird die Lösung basisch; pH
    = berechenbar = 11.44 bzw. 12.17. <br>

    $\ce{
    CO3^2- + H2O -> HCO3- + OH-
    }$ <br>

    <u>Eine solche leicht zugängliche, stabile Massloesung heißt</u>
    <w>Urtiter: stabile Massloesung über lange Zeit. z.B. $\ce{Na2CO3}$ zur Gehaltsbestimmung von
        Titrierfluessigkeiten z.B. anderen Massloesungen</w> <br>

    Davon gibt es nicht viele, weil über die Zeit Reaktionen ablaufen können, die den pH ändern, <u>z.B. gibt
        Salzsäure $\ce{HCl}$-Gas ab</u>, oder <u>Laugen greifen das Glas</u> an oder mit Luft-$\ce{CO2}$ bildet
    sich beim Lösen Kohlensäure die $\ce{OH-}$ verbraucht. <u>Permanganat ist stark oxidierend</u>, das jedes
    "Staubteilchen" oxidiert wird und sich die Lösung verbraucht; frisch herstellen. Titriplex ist recht stabil!<br>

    <w>Normalloesung: Massloesung mit Bezug auf die Normalität statt Molarität</w> Im Prinzip veraltet nach aktueller
    Nomenklatur aber in der Analytik oder bei Säure/Base-Reaktionen noch gängig berücksichtigt (??) wie eine Substanz
    reagiert. z.B. $\ce{HCl(aq)}$ hat 1 Proton; reagiert 1:1 mit $\ce{NaOH}$; Molarität und Normalität sind gleich.
    Aber 1 m $\ce{H2SO4}$ hat 2 Protonen d.h. die doppelte Menge $\ce{NaOH}$ reagiert bis zur Neutralität \-> 1 m
    $\ce{H2SO4}$ = 2 m $\ce{H3SO4}$ oder 1 n $\ce{H2SO4}$ ist 0.5 m wird mit 1n $\ce{NaOH}$ (1 m) neutralisiert.<br>
    <wd>
        Normalität: berücksichtigt die Reaktion und Stoichiometrie einer gelösten Substanz.<br>
        Beispiel:<br>
        $\ce{H2SO4(aq)}$ mit $m = 0.5\textrm{ mol / L}$ ist $n = 1\textrm{ mol / L}$ bezüglich Protonen. <br>
        $\ce{MnO4-}$ (Permanganat-Ion) mit $m = 0.2\textrm{mol / L}$ ist $n = 1\textrm{mol / L}$ bezüglich der
        Elektronen der Redox-Reaktion <br>
    </wd> <br>
    Für die Rechnung ist die Normalität also einfacher weil man sich die Reaktion nicht anschauen muss und die
    Äquivalenzbedingung 1:1 ist. <br>
    <wd>
        Faktoren auf Flaschen: F; Änderung durch Alterung der Massloesung zur <u>Brechnung</u> notieren! \-> nur noch
        $\ce{Na2S2O3}$ bei $\ce{Cu^2+}$ iodometrisch <br>
        <u>Beispiel:</u> $\ce{NaOH}$ wird mit $\ce{HCl(aq)}$ titriert. <br>
        \-> Äquivalenzbedingung: 1mol $\ce{H+}$ $\hat{=}$ 1mol $\ce{OH-}$ <br>
        Massloesung: $c=0.1$ m $\ce{HCl(aq)}$ mit Faktor F <br>
        Stoffmenge: $n_{\ce{HCl}} = c_{\ce{HCl}} \cdot V_{\ce{HCl}} = F \cdot 0.1 \mathrm{ \frac{mol}{L}} \cdot
        V_{\ce{HCl}}\mathrm{\color{red}{\ (der\ Bürette)}} \mathrm{\ mL} \cdot 10^{-3} \mathrm{ \frac{L}{mL}} =
        n({\ce{OH-}})$ <br>
    </wd>

</p>
