package internal

import (
	"os"
	"strconv"

	"github.com/olekukonko/tablewriter"
)

func List() error {
	entries, err := LoadMetadata()
	if err != nil {
		return err
	}

	table := tablewriter.NewWriter(os.Stdout)
	table.Header([]string{"ID", "Original Path", "Deleted At", "Dir"})

	for _, e := range entries {
		table.Append([]string{
			e.ID,
			e.OriginalPath,
			e.DeletedAt.Format("2006-01-02 15:04:05"),
			strconv.FormatBool(e.IsDir),
		})
	}

	table.Render()
	return nil
}
