# config valid only for current version of Capistrano
lock '3.6.1'

set :application, 'linkedlist'
set :repo_url, 'git@github.com:wezm/pkb.git'

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
set :linked_files, fetch(:linked_files, []).push('config/settings.yml', 'config/secrets.yml')

# Default value for linked_dirs is []
set :linked_dirs, fetch(:linked_dirs, []).push('log', 'tmp/pids', 'tmp/cache', 'tmp/sockets', 'vendor/bundle', 'public/system', 'pages')

set :bundle_without, [:development, :test]

# Default value for default_env is {}
# set :default_env, { path: "/opt/ruby/bin:$PATH" }

# Default value for keep_releases is 5
# set :keep_releases, 5

namespace :deploy do
  task :symlink_dash_snippets do
    on roles(:app) do
      execute "if [ ! -f #{shared_path}/pages/dash-snipets.html ]; then ln -sf #{shared_path}/pages/dash-snippets.html #{release_path}/public/dash-snippets.html; fi"
    end
  end

  before :updated, :symlink_dash_snippets
end

