---
layout: page
permalink: /publications/
title: Publications
description:
years: [2023, 2022, 2021, 2020, 2019, 2018, 2017, 2016, 2015, 2014, 2013, 2012, 2011, 2010, 2009, 2008, 2007]
nav: true
nav_order: 1
---
<!-- _pages/publications.md -->
<div class="publications">

{%- for y in page.years %}
  <h2 class="year">{{y}}</h2>
  {% bibliography -f papers -q @*[year={{y}}]* %}
  {% bibliography -f conferences -q @*[year={{y}}]* %}
  {% bibliography -f chapters -q @*[year={{y}}]* %}
{% endfor %}

</div>


### Tutorials
- [Succinct Data Structures in Information Retrieval: Theory and Practice](/assets/pdf/papers/TUTORIAL@SIGIR2016.pdf). Full-day tutorial at [ACM Sigir 2016](http://sigir.org/sigir2016/) with [Simon Gog](https://algo2.iti.kit.edu/gog/)