(ns ncube.subscriptions
  (:require [re-frame.core :refer [reg-sub]]))

(reg-sub
 :current-route
 (fn [db]
   (:current-route db)))

(reg-sub
 :workspaces
 (fn
   [db]
   (:workspaces db)))
