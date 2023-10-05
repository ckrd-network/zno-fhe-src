#!/bin/bash

    # This script first collects all the unique base filenames by stripping off the patterns.
    # Before trying to concatenate the files, the script checks if each of the paired files exists.
    #   - If both exist, it merges them;
    #   - if only one exists, it copies just that one.
    #   - If neither exists, it skips to the next base filename.
    # For each unique base filename, it merges the contents of its paired files.
    # Then, using sed, it removes the optional dot format and layout lines and specific field/data pairs.
    #   - Specific matching on the 'edge' and 'node' removal to ensure it only removes lines ending in a semicolon ;.
    #   - regex pattern to remove fields whose values are enclosed in double quotes.
    #   - Two passes of the sed command to remove the specified field/data pairs. This avoids leaving a trailing comma:
    #       - first, if they're followed by a comma (indicating they're not the last item), and
    #       - second, if they're preceded by a comma (indicating they are the last item before the closing bracket).
    # The node naming convention is also adjusted.
    # Finally, it renames each processed file, prefixing its name with the number of characters it contains.

# Enable error reporting options
set -euo pipefail  # Exit on error and undefined variable, and prevent unintentional globbing or word splitting.

dir_src=$(pwd)/zno-helib-src/scripts/dxy/latex
dir_dst=$(pwd)/zno-helib-src/scripts/dot

pat1=__coll__graph.dot
pat2=__inherit__graph.dot

# Create an array of unique base filenames
declare -A unique_files

# Populate unique_files
for file in "${dir_src}"/*"${pat1}" "${dir_src}"/*"${pat2}"; do
    base=$(echo "$file" | sed -E "s/${pat1}|${pat2}//")
    unique_files["$base"]=1
done

# Process each unique base file
for base in "${!unique_files[@]}"; do
    outfile="${dir_dst}/$(basename "${base}").dot"

    # Check if files exist before merging
    if [[ -e "${base}${pat1}" && -e "${base}${pat2}" ]]; then
        cat "${base}${pat1}" "${base}${pat2}" > "$outfile"
    elif [[ -e "${base}${pat1}" ]]; then
        cat "${base}${pat1}" > "$outfile"
    elif [[ -e "${base}${pat2}" ]]; then
        cat "${base}${pat2}" > "$outfile"
    else
        echo "Neither paired files for $base exist. Skipping..."
        continue
    fi

    # Remove optional format and layout info
    sed -i '/edge \[.*\];/d' "$outfile"
    sed -i '/node \[.*\];/d' "$outfile"

    # Remove specified field/data pairs
    sed -i -E 's/(height|width|fontsize|URL)=[^ ,"]+//g' "$outfile"
    sed -i -E 's/(height|width|fontsize|URL)="[^"]+"//g' "$outfile"

    # Remove multiple consecutive commas, replace with single comma, and cleanup hanging commas before closing brackets or semicolons
    sed -i 's/,\+/,/g' "$outfile"
    sed -i 's/,\s*\]/\]/g' "$outfile"
    sed -i 's/,\s*;/;/g' "$outfile"
    sed -i 's/\[,//g' "$outfile"

    # Change node naming convention
    sed -i 's/Node/N/g' "$outfile"

    # Rename files with character count prefix
    len=$(wc -c <"$outfile" | tr -d ' ')
    mv "$outfile" "${dir_dst}/${len}__$(basename "${base}").dot"
done

# # Set bash error reporting options
# set -euo pipefail

# # Variables
# dir_src=$(pwd)/zno-helib-src/scripts/dxy/latex
# dir_dst=$(pwd)/zno-helib-src/scripts/dot

# pat1=__coll__graph.dot
# pat2=__inherit__graph.dot

# # Merge files
# for src_file in "$dir_src"/*"$pat1"; do
#     base_name=$(basename "$src_file" "$pat1")
#     second_file="$dir_src/$base_name$pat2"

#     # Check if the second file exists
#     if [[ -f "$second_file" ]]; then
#         dst_file="$dir_dst/${base_name}.dot"
#         cat "$src_file" "$second_file" > "$dst_file"
#     else
#         echo "Skipping $src_file as its pair $second_file doesn't exist."
#     fi
# done

# # Process merged files
# for file in "$dir_dst"/*.dot; do
#     # Remove optional format/layout information
#     sed -i -e '/^edge /d' -e '/^node /d' "$file"

#     # Remove field/data for height, width, fontsize, and URL
#     sed -i -e '/height=/d' -e '/width=/d' -e '/fontsize=/d' -e '/URL=/d' "$file"

#     # Change node name convention
#     sed -i 's/Node\([0-9]\+\)/N\1/g' "$file"

#     # Rename file with number of characters at the start
#     file_length=$(wc -c <"$file")
#     mv "$file" "$dir_dst/${file_length}__$(basename "$file")"
# done

# # # Error reporting options
# # set -euo pipefail
# # # Uncomment the next line for tracing the commands
# # # set -x

# # dir_src=$(pwd)/zno-helib-src/scripts/dxy/latex
# # dir_dst=$(pwd)/zno-helib-src/scripts/dot

# # pat1=__coll__graph.dot
# # pat2=__inherit__graph.dot

# # # Step 1: Merge pairs of files into a single file.
# # for file in "$dir_src"/*"$pat1"; do
# #     # Get the base name without the pattern and the extension
# #     base_name=$(basename "$file" "$pat1")

# #     # If the paired file exists, then merge
# #     if [[ -f "$dir_src/$base_name$pat2" ]]; then
# #         cat "$file" "$dir_src/$base_name$pat2" > "$dir_dst/$base_name.dot"
# #     fi
# # done

# # # Step 2: Remove lines starting with 'edge' or 'node'.
# # # Step 3: Remove specified field names and their data.
# # # Step 4: Rename the merged files to include the number of characters they contain.
# # for file in "$dir_dst"/*.dot; do
# #     # Using sed for removing lines and specific patterns
# #     sed -i -e '/^edge/d' \
# #            -e '/^node/d' \
# #            -e 's/height=[^ ]*//g' \
# #            -e 's/width=[^ ]*//g' \
# #            -e 's/fontsize="10"//g' \
# #            -e 's/URL="[^"]*"//g' "$file"

# #     # Count characters in file
# #     char_count=$(wc -c < "$file")

# #     # Rename the file
# #     mv "$file" "$dir_dst/${char_count}__$(basename "$file")"
# # done

# # # # Enable error reporting options
# # # set -euxo pipefail

# # # dir_src=$(pwd)/zno-helib-src/scripts/dxy/latex
# # # dir_dst=$(pwd)/zno-helib-src/scripts/dot

# # # pat1=__coll__graph.dot
# # # pat2=__inherit__graph.dot

# # # # For each matching pair
# # # for file in "${dir_src}"/*"${pat1}"; do
# # #     # Extract base name from file name
# # #     base_name=$(basename "$file" "${pat1}")

# # #     # Construct the counterpart filename
# # #     counterpart="${dir_src}/${base_name}${pat2}"

# # #     # Check if the counterpart exists, if yes, merge them
# # #     if [ -f "${counterpart}" ]; then
# # #         cat "${file}" "${counterpart}" > "${dir_dst}/${base_name}.dot"
# # #     fi
# # # done

# # # # Remove optional DOT fields
# # # # NOTE: This example assumes that optional fields are lines starting with 'edge' or 'node'.
# # # # Adjust the sed command according to the actual format/layout information.
# # # for merged_file in "${dir_dst}"/*.dot; do
# # #     sed -i '/^edge/d' "${merged_file}"
# # #     sed -i '/^node/d' "${merged_file}"
# # # done

# # # # Rename the files with character count
# # # for merged_file in "${dir_dst}"/*.dot; do
# # #     char_count=$(wc -c < "${merged_file}")
# # #     mv "${merged_file}" "${dir_dst}/${char_count}__$(basename "${merged_file}")"
# # # done


# # # # dir_src=$(pwd)/zno-helib-src/scripts/dxy/latex
# # # # dir_dst=$(pwd)/zno-helib-src/scripts/dot
# # # # pat1=__coll__graph.dot
# # # # pat2=__inherit__graph.dot
# # # # pushd ${dir_src} || return
# # # #   # Find files with the pattern *__coll__graph.dot
# # # #   for file in *${pat1}; do
# # # #       # Remove the __coll__graph.dot suffix to get the base name
# # # #       base_name="${file%${pat1}}"

# # # #       # Determine the name of the second file with pattern *__inherit__graph.dot
# # # #       second_file="${base_name}${pat2}"

# # # #       # Check if the second file exists
# # # #       if [[ -f "$second_file" ]]; then
# # # #           # Concatenate the contents of both files and redirect to a new file
# # # #           cat "$file" "$second_file" > "../../dot/${base_name}.dot"
# # # #       else
# # # #           # If the second file doesn't exist, just copy the first file
# # # #           cp "$file" "../../dot/${base_name}.dot"
# # # #       fi
# # # #   done

# # # #   # Handle files which only have the *__inherit__graph.dot pattern and not the *__coll__graph.dot pattern
# # # #   for file in *${pat2}; do
# # # #       # Remove the __inherit__graph.dot suffix to get the base name
# # # #       base_name="${file%${pat2}}"

# # # #       # Determine the name of the possible first file with pattern *__coll__graph.dot
# # # #       first_file="${base_name}${pat1}"

# # # #       # Check if the first file does not exist (since if it did, it would've been handled in the first loop)
# # # #       if [[ ! -f "$first_file" ]]; then
# # # #           cp "$file" "../../dot/${base_name}.dot"
# # # #       fi
# # # #   done
# # # # popd

# # # # # Find unique prefixes by removing the _x and _y suffixes and sorting/uniq-ing the result
# # # # # for prefix in $(ls $(pwd)/zno-fhe-src/scripts/dxy/latex/*_pair_*.dot | sed 's/_pair_[xy]\.dot//g' | sort | uniq); do
# # # # #     # Concatenate the contents of the paired files into a new file
# # # # #     cat "${prefix}__coll__graph.dot" "${prefix}__inherit__graph.dot" > "$(pwd)/zno-fhe-src/scripts/dot/${prefix}_pair.dot"
# # # # # done
