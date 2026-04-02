package internal

import (
	"fmt"
	"github.com/charmbracelet/log"
	"os"
	"path/filepath"
	"time"
)

func Move(paths []string, recursive, force, verbose bool) error {
	err := EnsureDirs()
	if err != nil {
		return err
	}

	entries, _ := LoadMetadata()

	for _, p := range paths {
		info, err := os.Stat(p)
		if err != nil {
			if force {
				continue
			}
			return err
		}

		if info.IsDir() && !recursive {
			return fmt.Errorf("can't trash %s: is a directory. Try with -r", p)
		}

		id := fmt.Sprintf("%d", time.Now().UnixNano())
		name := filepath.Base(p)
		stored := id + "_" + name
		dest := filepath.Join(FilesDir(), stored)

		err = os.Rename(p, dest)
		if err != nil {
			return err
		}

		abs, _ := filepath.Abs(p)

		entries = append(entries, Entry{
			ID:           id,
			OriginalPath: abs,
			StoredName:   stored,
			DeletedAt:    time.Now(),
			IsDir:        info.IsDir(),
		})

		if verbose {
			log.Info("trashed file", "file", p)
		}
	}

	return SaveMetadata(entries)
}
