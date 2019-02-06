# pkb — Personal Knowledge Base

pkb is a small Rails application that allows you to efficiently publish a 
collection of Markdown files.

## Configuration

* Copy the `config/settings.yml.sample` file to `config/settings.yml` and fill
  in your own details
* Run `bundle install`
* Link the directory with your Markdown files in it. E.g. `ln -s ~/Dropbox/My\ Markdown\ Files pages`
* Start the server, `rails s` and visit [http://localhost:3000/pages](http://localhost:3000/pages)
* You should have Markdown file called `home.md`. This file will be shown as
  the homepage: [http://localhost:3000/](http://localhost:3000/)

## Deployment

pkb is designed to be deployed behind a caching proxy such as varnish. There is
a sample varnish configuration in `config/varnish.vcl`. There is a script for
building a Docker image: `bin/docker-build`.

The resulting image can be run something like this:

    sudo docker run -it --rm -e 'SECRET_KEY_BASE=asdfasdfasdf' -p 3000:3000/tcp  -v /home/wmoore/Projects/pkb/pages:/home/pkb/pages:ro 51d800a7496b
