# Browser-Based Game System

A lightweight game system designed for progressive user engagement. Players can start immediately with minimal commitment and choose their preferred way to preserve progress later.  [Connecting to people](docs/connecting-to-people.md) will be just as easy.


Basically, free plan gets browser storage of game data and friends data, Premium memberships add cloud storage.


## Features

### Quick Start
- Begin playing instantly by just entering a name
- Automatic progress saving in browser cache
- No initial account creation or email required

### Storage Options

#### Free Tier
- Automatic browser cache storage
- Email-based backup system
  - Request a backup file anytime
  - Easy restore on any browser
  - No account needed

#### Premium Tier
- Real-time cloud synchronization
- Seamless multi-browser experience
- Automatic backup and restore

## System Flow

The diagram below shows how users progress from quick start to managed storage:

```mermaid
flowchart TD
    C[Open New Browser] --> A[Quick Start: Enter Name]
    C --> B[Return User: Load Data File]
    A --> M[Save Profile in Browser Cache]
    M --> D[Start Game]
    B --> |Load Profile + Progress| D
    D --> E[Update Profile]
    D --> F[Make Game Progress]
    E --> G[Update Cached Data]
    F --> G
    
    subgraph Storage Endpoints
        H[(Browser Cache)]
        O[(Cloud Storage)]
        L[(Local Backup File)]
    end
    
    G --> H
    G --> N{Has Premium Account?}
    N --> |Yes| O
    N --> |No| J{Profile Has Email?}
    J --> |Yes| I[Email Backup File]
    J --> |No| K[Prompt for Email]
    K --> I
    I --> L
    L --> |Later: Resume on New Browser| B
```

## How It Works

1. **New Players**
   - Enter name and start playing
   - Progress automatically saved to browser cache
   - No registration required

2. **Saving Progress**
   - All gameplay automatically saved to browser cache
   - Choose backup method when ready:
     - Free: Request backup file via email
     - Premium: Enable cloud synchronization

3. **Resuming on New Browser**
   - Free users: Upload backup file
   - Premium users: Automatic sync
   - Instant access to all previous progress

## Design Philosophy

The system is built around progressive engagement:
- Zero friction to start playing
- Automatic local saving
- Optional backup methods
- Choice between simplicity (email backup) and convenience (cloud sync)

## Development Status

This project is under active development. Check our [Development Plan](docs/development-plan.md) for details on:
- Current progress
- Upcoming features
- Testing strategy
- Release timeline

## Getting Started

[Coming soon]