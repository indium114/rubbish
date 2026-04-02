package internal

import (
	"encoding/json"
	"os"
	"time"
)

type Entry struct {
	ID           string    `json:"id"`
	OriginalPath string    `json:"original_path"`
	StoredName   string    `json:"stored_name"`
	DeletedAt    time.Time `json:"deleted_at"`
	IsDir        bool      `json:"is_dir"`
}

func LoadMetadata() ([]Entry, error) {
	path := MetadataPath()

	if _, err := os.Stat(path); os.IsNotExist(err) {
		return []Entry{}, nil
	}

	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	var entries []Entry
	err = json.Unmarshal(data, &entries)
	return entries, err
}

func SaveMetadata(entries []Entry) error {
	data, err := json.MarshalIndent(entries, "", "  ")
	if err != nil {
		return err
	}
	return os.WriteFile(MetadataPath(), data, 0644)
}
