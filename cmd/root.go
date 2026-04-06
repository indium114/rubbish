package cmd

import (
	"context"
	"fmt"
	"os"

	"github.com/charmbracelet/fang"
	"github.com/indium114/rubbish/internal"
	"github.com/spf13/cobra"
)

var (
	flagRecursive bool
	flagForce     bool
	flagVerbose   bool
	flagPermanent bool

	flagList    bool
	flagRestore string
	flagDelete  string
	flagClear   bool
)

var rootCmd = &cobra.Command{
	Use:   "rubbish [files...]",
	Short: "A CLI file trash tool. A replacement for rm",
	Args:  cobra.ArbitraryArgs,
	RunE: func(cmd *cobra.Command, args []string) error {
		switch {
		case flagList:
			return internal.List()

		case flagRestore != "":
			return internal.Restore(flagRestore)

		case flagDelete != "":
			return internal.Delete(flagDelete)

		case flagClear:
			return internal.Clear()

		default:
			if len(args) == 0 {
				return fmt.Errorf("no files provided")
			}
			return internal.Move(args, flagRecursive, flagForce, flagVerbose, flagPermanent)
		}
	},
}

func Execute() {
	err := fang.Execute(context.Background(), rootCmd)
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	rootCmd.Flags().BoolVarP(&flagRecursive, "recursive", "r", false, "recursive")
	rootCmd.Flags().BoolVarP(&flagForce, "force", "f", false, "force")
	rootCmd.Flags().BoolVarP(&flagVerbose, "verbose", "v", false, "verbose")
	rootCmd.Flags().BoolVarP(&flagPermanent, "permanent", "p", false, "permanent")

	rootCmd.Flags().BoolVar(&flagList, "list", false, "list rubbish")
	rootCmd.Flags().StringVar(&flagRestore, "restore", "", "restore by id")
	rootCmd.Flags().StringVar(&flagDelete, "delete", "", "delete permanently by id")
	rootCmd.Flags().BoolVar(&flagClear, "clear", false, "clear rubbish bin")
}
