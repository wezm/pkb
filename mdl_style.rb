all

# Trailing punctuation in header (allow question mark)
rule 'MD026', :punctuation => ".,;:!"

exclude_rule 'MD002' # First header should be a h1 header
exclude_rule 'MD034' # Bare URL used
exclude_rule 'MD013' # Line length

