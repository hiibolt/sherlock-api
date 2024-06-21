### Sherlock API Docker Image

This repository provides a Docker Image for running a Sherlock API, which allows searching for usernames across various social networks and websites. The Docker Image can be found at `ghcr.io/hiibolt/sherlock-api`.

#### Environment Variables

- **PORT**: Specifies the port on which the API will listen.
- **PROXY_LINK**: Specifies the SOCKS5 proxy link to use for making requests (required for some sites).

#### Default Blacklisted Sites

The API includes a default set of blacklisted sites that do not work well with Sherlock:

- Oracle, 8tracks, Coders Rank, Fiverr, HackerNews, Modelhub, metacritic, xHamster, CNET, YandexMusic, HackerEarth, OpenStreetMap, Pinkbike, Slides, Strava, Archive, CGTrader, G2G, NationStates, IFTTT, SoylentNews, hunting, Contently, Euw, OurDJTalk, BitCoinForum, HEXRPG, Polymart, Linktree, GeeksforGeeks, Kongregate, RedTube, APClips, Heavy-R, RocketTube, Zhihu, NitroType, babyRU

#### Routes

The API provides two main routes:

1. **WebSocket Route (`/ws`)**:
   - Allows WebSocket connections to receive real-time updates as results are found.
   - Example CURL request:
     ```bash
     curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" -H "Host: your-host" -H "Origin: your-origin" -H "Sec-WebSocket-Key: SGVsbG8sIHdvcmxkIQ==" -H "Sec-WebSocket-Version: 13" http://your-api/ws
     ```

2. **Static Lookup Route (`/static`)**:
   - Accepts POST requests with a JSON body containing a username to search.
   - Example CURL request:
     ```bash
     curl -X POST -H "Content-Type: application/json" -d '{"username": "example_username"}' http://your-api/static
     ```

#### Docker Setup

To run the Sherlock API Docker container:

1. **Pull the Docker Image**:
   ```bash
   docker pull ghcr.io/hiibolt/sherlock-api
   ```

2. **Run the Docker Container**:
   ```bash
   docker run -d -p <your-port>:<container-port> -e PORT=<your-port> -e PROXY_LINK=<your-proxy-link> ghcr.io/hiibolt/sherlock-api
   ```

   Replace `<your-port>` with the port number you want to use and `<your-proxy-link>` with your SOCKS5 proxy link.

#### Example Docker Run Command

```bash
docker run -d -p 8080:8080 -e PORT=8080 -e PROXY_LINK="socks5://user:password@proxy-host:proxy-port" ghcr.io/hiibolt/sherlock-api
```

This sets up the Sherlock API Docker container to run on port 8080 with SOCKS5 proxy support.

Feel free to explore and integrate this Docker Image into your applications for username searching across multiple platforms, and credit me @hiibolt if possible!
