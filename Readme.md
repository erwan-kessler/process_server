# Subject

Il s'agit de faire un petit serveur web qui examine la liste des processus qui s'exécutent sur la machine.
Il devra supporter soit Linux, soit Windows (selon tes préférences).

Il possède quatre endpoints:

- POST `/acquire_process_list`:
  > Fait la liste des processus qui tournent actuellement sur la machine et les garde pour être récupérés plus tard.
  
  * Le corps de la réponse doit être vide.
  * Le but de cet endpoint est de peupler un cache qui sera utilisé par les endpoints suivants.


- GET `/processes`:

  > Récupère la liste des processus qui ont été préalablement stockés via l'endpoint `/acquire_process_list`.
     * Le corps de la réponse doit contenir un tableau JSON sous la forme suivante:

    ```json
    [
      {
        "pid": 674,
        "name": "synapse",
        "uid": 100,
        "username": "user"
      }
    ]
    ```
    * Si `/acquire_process_list` n'a pas été appelé au préalable, la requête doit retourner un tableau vide.
    * Si cet endpoint est appelé plusieurs fois sans /acquire_process_list entre les appels, la même liste doit être
      retournée.

    


- GET `/search`:
  > Renvoie la liste des processus, filtré.
    * Cet endpoint prends deux paramètres dans la query string:
        - pid=`<num>`: Filtre la liste pour ne retourner que les informations du process ayant le PID donné.
        - username=`<str>`: Filtre la liste pour ne retourner que les informations des process ayant le nom d'exécutable
          donné.
    * Si les paramètres sont combinés, ils agissent comme un AND (ex : ils ne retournent que les processus ayant le pid
      et le nom donné).


- GET `/data`:
  > Un endpoint SSE (Server-Sent Event) qui retourne les nouveaux processus trouvés via `/acquire_process_list` lorsque
  celui-ci est appelé.

  (Va voir du côté du module warp::filters::sse pour voir comment faire un endpoint long-running.)

    * Les process doivent etre dédupliqué, e.g. si plusieurs appels sont fait à /acquire_process_list, le même process
      ne doit pas apparaitre plusieurs foit dans /data.

    * Par exemple:

  ```shell
  curl http://localhost:8080/data &
  curl http://localhost:8080/acquire_process_list
  ping google.com &
  curl http://localhost:8080/acquire_process_list
  ```

    * Lorsque le premier `/acquire_process_list` est appelé, `/data` doit afficher tous les process qui tournent
      actuellement.
    * Le deuxième `/acquire_process_list` doit uniquement afficher les nouveaux processus, comme ping.

    * Chaque évènement doit être un objet JSON sous la forme:

    `{ "pid": 674, "name": "synapse", "uid": 1, "username": "user" }`

## Notes:

Nous conseillons l'utilisation de warp pour le serveur web, c'est une librairie de serveur HTTP très simple et très
facile à prendre en main. Libre à toi d'en utiliser une autre si tu préfères.