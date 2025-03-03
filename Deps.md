# Document : Besoins d'Hébergement pour le Projet

---

## Partie 1 : Hébergement Back Office / Front Office

### 1.1 Serveur Back Office / Front Office

| **Composant**   | **Description**                                                                                       | **Technologie**              | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|------------------------------|-------------------------------------|
| **Back Office**  | Gestion des données, utilisateurs, administration.                                                    | Apache, MySQL                | Processeur multi-coeurs, 16 GB RAM |
| **Front Office** | Interface utilisateur, compilation et distribution des chatbots.                                       | PM2, Webpack, CDN             | 4 à 8 coeurs, 32 GB RAM            |
| **Responsabilité de PM2** | Supervision du processus de build des chatbots, compilation avec Webpack, et stockage sur CDN. | PM2, Webpack                  | CPU + RAM selon la complexité      |

### 1.2 Capacité Serveur

| **Ressource**   | **Estimation**                                                                                         | **Commentaire**               |
|-----------------|-------------------------------------------------------------------------------------------------------|-------------------------------|
| **CPU**         | 4 à 8 coeurs                                                                                           | Selon le nombre de builds simultanés. |
| **RAM**         | 16 à 32 GB                                                                                             | Selon les besoins en mémoire pour la compilation et le stockage temporaire. |
| **Stockage**    | 500 GB à 1 TB                                                                                          | Pour les données des chatbots et les bases MySQL. |
| **Réseau**      | Bande passante de 100 Mbps à 1 Gbps                                                                    | S'assurer d'une bonne connectivité au CDN. |

---

## Partie 2 : Serveur WebSocket (Socket.io)

### 2.1 Infrastructure WebSocket

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **WebSocket**   | Gestion de la communication en temps réel avec les clients.                                            | Socket.io                      | 4 à 8 coeurs, 8 GB RAM             |
| **Capacité actuelle** | Actuellement, un serveur avec 4 coeurs et 8 Go de RAM supporte 150 clients sans problème.        | Socket.io                      | 150 utilisateurs simultanés        |
| **Scaling**     | Ajout de serveurs à la volée en fonction des besoins via un load balancer.                             | Socket.io, Nginx (Load Balancer)| Scalabilité automatique            |

### 2.2 Capacité Serveur

| **Ressource**   | **Estimation**                                                                                         | **Commentaire**               |
|-----------------|-------------------------------------------------------------------------------------------------------|-------------------------------|
| **CPU**         | 4 à 8 coeurs                                                                                           | Selon le nombre d'utilisateurs simultanés. |
| **RAM**         | 8 à 16 GB                                                                                              | 8 GB suffisent pour 150 utilisateurs, ajustable selon la charge. |
| **Réseau**      | 100 Mbps à 1 Gbps                                                                                      | Important pour les communications WebSocket en temps réel. |
| **Load Balancing** | Utilisation de load balancers pour ajouter dynamiquement des serveurs WebSocket en fonction de la demande. | Nginx, HAProxy ou équivalent. | Permet de gérer des pics de charge. |

---

## Partie 3 : Hébergement pour les Modèles LLM (LLaMA / Mistral)

### 3.1 Infrastructure pour les Modèles LLM

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **LLM Docker**  | Conteneurs pour héberger les modèles LLaMA/Mistral.                                                    | Docker                         | CPU : Core i7, RAM : 16 GB         |
| **Capacité**    | Chaque instance Docker supporte jusqu'à 5 clients simultanés.                                           | Docker, Load Balancer           | Scalabilité automatique            |
| **Scaling**     | Démarrage automatique des conteneurs supplémentaires selon la charge utilisateur.                       | Docker Swarm, Kubernetes        | Gestion dynamique des ressources   |

### 3.2 Capacité Serveur

| **Ressource**   | **Estimation**                                                                                         | **Commentaire**               |
|-----------------|-------------------------------------------------------------------------------------------------------|-------------------------------|
| **CPU**         | 1 Core i7 par instance                                                                                 | Nécessaire pour les calculs IA.|
| **RAM**         | 16 GB par instance                                                                                     | 16 GB pour les besoins en mémoire des modèles. |
| **Stockage**    | 100 à 500 GB (selon les modèles et données d'entraînement)                                              | Stockage pour les modèles et les logs. |
| **Load Balancing** | Gestion des utilisateurs par load balancer pour rediriger les clients vers des instances LLM disponibles. | Nginx, HAProxy ou équivalent. | Optimisation de la performance. |

---

## Partie 4 : Base de Données PostgreSQL avec pgvector

### 4.1 Infrastructure PostgreSQL avec pgvector

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **PostgreSQL**  | Base de données relationnelle avec extension pgvector pour gérer les données vectorielles.             | PostgreSQL, pgvector           | CPU multi-coeurs, RAM selon charge |
| **Scaling**     | Capacité de scaling vertical (CPU, RAM) et horizontal (réplication ou sharding).                       | PostgreSQL, Patroni, pgpool    | Gestion dynamique de la base       |
| **Optimisation** | Indexation des vecteurs pour recherche rapide. Surveillance de la charge et performance via outils comme Prometheus. | pgvector, Prometheus           | Optimisation des index             |

### 4.2 Capacité Serveur

| **Ressource**   | **Estimation**                                                                                         | **Commentaire**               |
|-----------------|-------------------------------------------------------------------------------------------------------|-------------------------------|
| **CPU**         | 4 à 8 coeurs                                                                                           | Selon le volume de données vectorielles. |
| **RAM**         | 16 à 32 GB                                                                                             | Pour gérer les index et les requêtes complexes. |
| **Stockage**    | 1 à 2 TB                                                                                               | Volume des données vectorielles et autres. |
| **Réseau**      | Connexion rapide pour assurer la réplication et la haute disponibilité.                                 | 1 Gbps recommandé.             |

---

## Partie 5 : Sécurisation et Monitoring

### 5.1 Sécurisation des Services

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **SSL/TLS**     | Chiffrement des communications pour les serveurs web et la base de données.                            | SSL, Let's Encrypt             | Chiffrement des connexions          |
| **Pare-feu**    | Configuration stricte pour limiter l'accès aux services.                                               | IPTables, UFW                  | Protection des serveurs            |
| **Accès sécurisé** | Limitation des accès via des rôles PostgreSQL et Docker.                                            | PostgreSQL, Docker              | Accès restreint pour la sécurité.   |

### 5.2 Monitoring et Maintenance

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **Monitoring**  | Surveillance des performances des serveurs et bases de données.                                        | Prometheus, Grafana            | Alertes sur la charge et la disponibilité. |
| **Logs**        | Gestion des logs pour suivre les incidents et ajuster les performances.                                | ELK Stack, Graylog             | Surveillance proactive.            |

---

## Partie 6 : Serveur pour l'Agent Python (Scraping)

### 6.1 Infrastructure pour l'Agent Python

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **Agent Python**| Serveur dédié à l'exécution des tâches de scraping de données.                                         | Python (Requests, BeautifulSoup, etc.) | CPU modéré, 2 à 4 coeurs |
| **Capacité**    | Tâches de scraping potentiellement intensives en termes de CPU et I/O disque.                          | 10 GB d'espace disque          | Gestion des résultats de scraping. |

### 6.2 Capacité Serveur

| **Ressource**   | **Estimation**                                                                                         | **Commentaire**               |
|-----------------|-------------------------------------------------------------------------------------------------------|-------------------------------|
| **CPU**         | 2 à 4 coeurs                                                                                           | Selon la complexité des tâches de scraping. |
| **RAM**         | 8 à 16 GB                                                                                              | RAM nécessaire pour les opérations de scraping. |
| **Stockage**    | 10 GB                                                                                                  | Principalement pour les fichiers de scraping temporaires et résultats. |

---

## Partie 7 : Microservice pour md-to-pdf

### 7.1 Infrastructure pour le Microservice md-to-pdf

**Description :** 
Le service **md-to-pdf** permet de convertir des fichiers Markdown en PDF avec des fonctionnalités avancées telles que les **templates personnalisés** et l'intégration d'une **API** pour une conversion dynamique.

| **Composant**   | **Description**                                                                                       | **Technologie**                | **Besoins**                         |
|-----------------|-------------------------------------------------------------------------------------------------------|--------------------------------|-------------------------------------|
| **md-to-pdf**   | Microservice de conversion Markdown vers PDF avec des templates et des polices améliorées.              | Rust             | CPU léger, 10 GB d'espace disque   |
| **API**         | API REST pour la conversion de Markdown en PDF avec support des templates CSS personnalisés.            | API POST Rust + reverse proxy apache                  | Serveur microservice.              |

### 7.2 Capacité Serveur

| **Ressource**   | **Estimation**                                                                                         | **Commentaire**               |
|-----------------|-------------------------------------------------------------------------------------------------------|-------------------------------|
| **CPU**         | 2 à 4 coeurs                                                                                           | Conversion des fichiers en PDF nécessitant peu de CPU. |
| **RAM**         | 4 à 8 GB                                                                                               | Conversion légère.             |
| **Stockage**    | 10 GB                                                                                                  | Pour stocker temporairement les fichiers Markdown et PDFs. |

---
