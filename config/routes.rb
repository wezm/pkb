Rails.application.routes.draw do
  # The priority is based upon order of creation: first created -> highest priority.
  # See how all your routes lay out with "rake routes".
  root 'pages#show', :id => 'home'

  resources :pages, :only => [:index]
  resources :tags, :only => [:index, :show]

  get '/sitemap', :to => 'site#sitemap', :defaults => {:format => 'xml'}, :as => 'sitemap'

  get '/:id', to: 'pages#show', id: /[a-z0-9-]+/, as: 'page'

end
