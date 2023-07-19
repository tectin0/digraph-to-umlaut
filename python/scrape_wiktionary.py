import requests


def main():
    """Scrape the wiktionary page for chemistry terms and save them to a file."""

    url = "https://de.wiktionary.org/wiki/Verzeichnis:Deutsch/Chemie/Fachwortliste"

    response = requests.get(url)

    response = response.text.split("Abschnitt bearbeiten: A")[1]
    response = response.split("Index:")[0]

    # example:
    # <li><a href="/w/index.php?title=Zersetzungsspannung&amp;action=edit&amp;redlink=1" class="new" title="Zersetzungsspannung (Seite nicht vorhanden)">Zersetzungsspannung</a></li>
    # <li><a href="/w/index.php?title=Ziegler-Katalysator&amp;action=edit&amp;redlink=1" class="new" title="Ziegler-Katalysator (Seite nicht vorhanden)">Ziegler-Katalysator</a></li>
    # get the title -> for example Zersetzungsspannung

    raw_titles = response.split('title="')

    for i in range(len(raw_titles)):
        raw_titles[i] = raw_titles[i].split('">')[0]

    titles: list[str] = []

    for title in raw_titles:
        if title.startswith("Abschnitt"):
            continue

        if title.endswith(" (Seite nicht vorhanden)"):
            title = title.split(" (Seite nicht vorhanden)")[0]

        if "&#39;" in title:
            title = title.replace("&#39;", "'")

        titles.append(title)

    with open("data/chem_terms.dic", "w", encoding="utf-8") as file:
        for title in titles:
            file.write(title + "\n")


if __name__ == "__main__":
    main()
