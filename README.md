# pkb -- Personal Knowledge Base

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
a sample varnish configuration in `config/varnish.vcl`. For getting it on to
the server Capistrano is recommended. There is a configuration that will need
adjusting in `config/deploy.rb` and `config/deploy/production.rb`.

