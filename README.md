# Rust File Organizer

## Description
This is a media file organizer. It will traverse a folder path and will move all media files inside the folder (recursively) to a pre-determined location (storage).
## Installation
## Usage
## Game Plan
### Design
- Will do the following
    - Accept a folder path
    - Recursively travel down path
        - If File:
            - figure out what kind of file it is
            - If its media file
                - Subject it through naming pipeline

    ``` EX:
        WESTWORLD.06.02.COOLRIP.2022.720p.mp4
        WESTWORLD.06.02.COOLRIP.2022.720p.mkv
        WESTWORLD.06.02.COOLRIP.2022.720p.srt
    ```

#### Naming Extractors
- ShowNameExtractor
- Season Extractor
- Episode Extractor
- Ripper Extractor
- Quality Extractor
- Year Extractor
- BitRate Extractor
---
- Pluck Out Desired Extractors Output and arrange in new name
    - Spits out a JSON of all applicable as well
- Create New Name + Move File to New Location
