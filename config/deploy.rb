# config valid only for current version of Capistrano
lock '3.4.0'

set :application, 'linkedlist'
set :repo_url, 'git@git.wezm.net:pkb.git'

# Default branch is :master
# ask :branch, `git rev-parse --abbrev-ref HEAD`.chomp

# set :use_sudo, false

# Default deploy_to directory is /var/www/my_app_name
set :deploy_to, -> { "/home/#{fetch :application}/www" }

set :ssh_options, {
  forward_agent: true,
}

# Default value for :scm is :git
# set :scm, :git

# Default value for :format is :pretty
# set :format, :pretty

# Default value for :log_level is :debug
set :log_level, :info

# Default value for :pty is false
# set :pty, true

# Default value for :linked_files is []
set :linked_files, fetch(:linked_files, []).push('config/database.yml', 'config/secrets.yml')

# Default value for linked_dirs is []
set :linked_dirs, fetch(:linked_dirs, []).push('log', 'tmp/pids', 'tmp/cache', 'tmp/sockets', 'vendor/bundle', 'public/system')

set :bundle_without, [:development, :test]

namespace :deploy do

  desc "Link pages directory"
  task :link_pages do
    on roles(:app) do |host|
      execute :ln, '-s', "~/Sync/#{fetch :application}", release_path + 'pages'
    end
  end

end


# Default value for default_env is {}
# set :default_env, { path: "/opt/ruby/bin:$PATH" }

# Default value for keep_releases is 5
# set :keep_releases, 5

after 'deploy:updated', 'deploy:link_pages'
