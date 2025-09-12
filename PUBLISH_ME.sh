bundle exec jekyll build
#rsync -avr _site/  a050143@pages.di.unipi.it:./public_html
rsync -avr -e "ssh -J rossano@xor.di.unipi.it" _site/ a050143@pages.di.unipi.it:./public_html