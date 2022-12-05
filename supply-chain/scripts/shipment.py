import sys
import pandas as pd

file = sys.argv[1]

# create dataframe
df = pd.read_csv(file)
df.drop(columns=df.columns[0], axis=1, inplace=True)
df = df.groupby('size').mean().reset_index()

print(df)