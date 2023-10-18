bundle exec jekyll build
rsync -av _site/  a050143@pages.di.unipi.it:./public_html
