#!/usr/bin/python
# -*- coding: latin-1 -*-

from pybtex.database.input import bibtex
import os.path

papers_dir = "/papers/"
template_filename = "research.markdown_template"
output_filename = "../research.markdown"
journals_bib = '../bibs/journals.bib'
conferences_bib = "../bibs/conferences.bib"


# TODO:

def get_author(person):
    first = ""
    l = str(person).split(",")
    for name in l[1].split():
        first += name[0] + ". "
    return first + l[0]

def get_authors(persons):
    authors = ""
    if len(persons) > 1:
        authors = ", ". join(get_author(person) for person in persons[:-1])
        if len(persons) > 2:
            authors += ","
        authors += " and "
    authors += get_author(persons[-1])
    return authors

def clean_text(text):
    text = text.replace("{\'a}", "à").replace("{\'e}", "é")
    return text.replace("{", "").replace("}", "")


def get_entry(k, data):
    #print(data)
    pdf_file =  "{}{}.pdf".format(papers_dir, k)
    pdf_link = "[![.pdf](/imgs/pdf.png)]({})".format(pdf_file)
    is_pdf = os.path.isfile(".."+ papers_dir + k + ".pdf")

    authors = clean_text(get_authors(data.persons['author']))
    title = clean_text(data.fields['title'])
    note = None
    try:
        note = data.fields['note']
    except:
        pass
    journal = ""
    try:
        journal = clean_text(data.fields['journal'])
    except:
        pass
    try:
        journal = clean_text(data.fields['booktitle'])
    except:
        pass
    year = 2000
    try:
        year = data.fields['year']
    except:
        print(k, "year is missing")
    doi_link = ""
    try:
        doi_link = "https://doi.org/{}".format(data.fields['doi'])
    except:
        pass
    string = "- "

    string += authors
    if is_pdf:
        string += "<br>" + "[*{}*]({})".format(title, pdf_file)
    else:
        string += "<br>" + "*{}*".format(title)
    string += "<br>" + "{},".format(journal)
    string += " " + "{}".format(year)
    if note:
        string += " " + "**" + note + "**"
    extra = []

    if len(doi_link):
        extra.append( "[![doi](/imgs/doi.png)]({})".format(doi_link))
    if is_pdf:
        extra.append(pdf_link + " ")
    if len(extra):
        string += "<br>" + " ".join(extra)

    return (int(year), string)

def get_biblio_render(bib_filename):
    parser = bibtex.Parser()
    bib_data = parser.parse_file(bib_filename)

    return "\n".join(e[1] for e in
                     sorted([get_entry(k, data) for k, data in bib_data.entries.items()], reverse = True))


template_text = open(template_filename, "r").read().replace("###Remove_me", "")

journals = get_biblio_render(journals_bib)
conferences = get_biblio_render(conferences_bib)

template_text = template_text.replace("###Journals_rep", journals)
template_text = template_text.replace("###Conferences_rep", conferences)

f = open(output_filename, "w")
f.write(template_text)
f.close()
