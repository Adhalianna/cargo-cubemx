A great deal of the work on the compiler was focused on finding correct data
structures for the problem. Initial version used only vectors and quite
detailed grammar. The grammar used by Pest was modified several times before it
reached its simple form. After that a hash map was added to the compiler
simplifying writing of match statements (Rust's equivalent of switch case) and
code in general. Next a nested structure of "properties" (name used in the
grammar) turned out to cause design difficulties. To solve them finally a
structure called patricia tree (in particalr a mapping based on patricia tree)
was used and the grammar was simplified even further moving a part of parsing away
from Pest.