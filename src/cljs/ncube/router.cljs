(ns ncube.router
  (:require
   [re-frame.core :refer [dispatch subscribe]]
   [reitit.core :as r]
   [reitit.frontend :as rf]
   [reitit.coercion.spec :as rss]
   [reitit.frontend.easy :as rfe]
   [ncube.onboarding.views :as onboarding]
   [ncube.views :refer [home-panel]]))

(def routes
  ["/"
   [""
    {:name :home
     :view home-panel}]
   ["onboarding"
    {:name :onboarding
     :view onboarding/panel}]])

(defn on-navigate
  [new-match]
  (when new-match
    (dispatch [:navigated new-match])))

(def router
  (rf/router
   routes
   {:data {:coercion rss/coercion}}))

(defn init-routes!
  []
  (js/console.log "initializing routes")
  (rfe/start!
   router
   on-navigate
   {:use-fragment false}))